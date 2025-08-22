use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::{Collateral, calculate_health_factor, get_lamports_from_usd, withdraw_sol, burn_tokens, SEED_CONFIG_ACCOUNT, Config, CustomError};


#[derive(Accounts)]

pub struct Liquidate<'info>{
    #[account(mut)]
    pub liquidator: Signer<'info>,

    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        has_one = sol_account,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = liquidator,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result <()> {
    let health_factor = calculate_health_factor (
        ctx.accounts.collateral_account.lamport_balance,
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    require!(
        health_factor < ctx.accounts.config_account.min_health_factor,
        CustomError::AboveMinHealthFactor,
    );

    let lamports = get_lamports_from_usd(&amount_to_burn, &ctx.accounts.price_update)?;
    let liquidation_bonus = lamports * ctx.accounts.config_account.liquidation_bonus / 100;
    let amount_to_liquidate = lamports + liquidation_bonus;


    withdraw_sol(
        ctx.accounts.collateral_account.bump_sol_account,
        &ctx.accounts.collateral_account.depositer,
        &ctx.accounts.system_program,
        &ctx.accounts.sol_account,
        &ctx.accounts.liquidator.to_account_info(),
        amount_to_liquidate,
    )?;

    burn_tokens(
        &ctx.accounts.token_program,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.liquidator,
        amount_to_burn,
    )?;

    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports();
    collateral_account.amount_minted -= amount_to_burn;

    calculate_health_factor(
        ctx.accounts.collateral_account.lamport_balance,
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update
    )?;

    Ok(())
}
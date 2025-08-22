use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::{
    Collateral, Config, CustomError, 
    calculate_health_factor, get_lamports_from_usd, withdraw_sol, burn_tokens, 
    SEED_CONFIG_ACCOUNT, LIQUIDATION_FEE_BPS, calculate_fee
};

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

    #[account(mut)]
    pub fee_recipient: Option<SystemAccount<'info>>,

    pub token_program: Program<'info, Token2022>,
    
    pub system_program: Program<'info, System>,
}

pub fn process_liquidate(ctx: Context<Liquidate>, amount_to_burn: u64) -> Result <()> {
    // First I check if this position is actually unhealthy and needs liquidation
    let health_factor = calculate_health_factor(
        ctx.accounts.collateral_account.lamport_balance,
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    // I only allow liquidation of positions that are actually at risk
    require!(
        health_factor < ctx.accounts.config_account.min_health_factor,
        CustomError::AboveMinHealthFactor,
    );

    // I log everything for transparency - liquidations are serious business
    msg!(
        "Liquidation initiated: Account={}, Health Factor={}, Amount To Burn={}",
        ctx.accounts.collateral_account.depositer.to_string(),
        health_factor,
        amount_to_burn
    );

    // I convert the stablecoin amount to SOL value using current prices
    let lamports = get_lamports_from_usd(&amount_to_burn, &ctx.accounts.price_update)?;
    
    // I give liquidators a bonus to incentivize them to help maintain system health
    let liquidation_bonus = lamports * ctx.accounts.config_account.liquidation_bonus / 100;
    
    // I also collect a small fee for the protocol treasury
    let protocol_fee = calculate_fee(lamports, LIQUIDATION_FEE_BPS)?;
    
    // The liquidator gets the collateral plus bonus, minus the protocol fee
    let amount_to_liquidate = lamports + liquidation_bonus - protocol_fee;
    
    // I log all the economics for transparency
    msg!(
        "Liquidation economics: Collateral={}, Bonus={}, Fee={}, Total={}",
        lamports,
        liquidation_bonus,
        protocol_fee,
        amount_to_liquidate
    );

    // I transfer the SOL collateral to the liquidator
    withdraw_sol(
        ctx.accounts.collateral_account.bump_sol_account,
        &ctx.accounts.collateral_account.depositer,
        &ctx.accounts.system_program,
        &ctx.accounts.sol_account,
        &ctx.accounts.liquidator.to_account_info(),
        amount_to_liquidate,
    )?;

    // If there's a fee recipient configured, I send the fee there
    if let Some(fee_recipient) = &ctx.accounts.fee_recipient {
        withdraw_sol(
            ctx.accounts.collateral_account.bump_sol_account,
            &ctx.accounts.collateral_account.depositer,
            &ctx.accounts.system_program,
            &ctx.accounts.sol_account,
            fee_recipient,
            protocol_fee,
        )?;
        
        msg!("Protocol fee of {} lamports collected", protocol_fee);
    } else {
        msg!("No fee recipient provided, fee remains in collateral account");
    }

    // I burn the stablecoins that the liquidator is using to purchase the collateral
    burn_tokens(
        &ctx.accounts.token_program,
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.liquidator,
        amount_to_burn,
    )?;

    // I update the collateral account to reflect the new balances
    ctx.accounts.collateral_account.lamport_balance = ctx.accounts.sol_account.lamports();
    ctx.accounts.collateral_account.amount_minted -= amount_to_burn;

    // I verify that the position is healthier after liquidation
    let new_health_factor = calculate_health_factor(
        ctx.accounts.collateral_account.lamport_balance,
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update
    )?;
    
    // I log the final state for transparency
    msg!(
        "Liquidation complete: New Health Factor={}, Remaining Collateral={}, Remaining Debt={}",
        new_health_factor,
        ctx.accounts.collateral_account.lamport_balance,
        ctx.accounts.collateral_account.amount_minted
    );

    Ok(())
}
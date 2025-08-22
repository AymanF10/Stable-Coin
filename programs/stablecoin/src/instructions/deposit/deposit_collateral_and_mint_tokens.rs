use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, Token2022}};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use crate::{
    Collateral, Config, CustomError,
    SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT, SEED_SOL_ACCOUNT, 
    check_health_factor, deposit_sol, mint_tokens, calculate_fee,
    MAX_MINT_AMOUNT, MINT_FEE_BPS, MIN_COLLATERAL_RATIO, MAX_COLLATERAL_RATIO
};

#[derive(Accounts)]
pub struct DepositCollateralAndMintTokens<'info> {
    #[account(mut)]
    pub depositer: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Box<Account<'info, Config>>, 

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = depositer, 
        space = 8 + Collateral::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositer.key().as_ref()],
        bump,
    )]
    pub collateral_account: Account<'info, Collateral>,
    
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT, depositer.key().as_ref()],
        bump,
    )]
    pub sol_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = depositer,
        associated_token::mint = mint_account,
        associated_token::authority = depositer,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub fee_recipient: Option<SystemAccount<'info>>,
    
    pub token_program: Program<'info, Token2022>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    pub price_update: Account<'info, PriceUpdateV2>
}

pub fn process_deposit_collateral_and_mint_tokens(
    ctx: Context<DepositCollateralAndMintTokens>,
    amount_collateral: u64,
    amount_to_mint: u64
) -> Result<()> {
    require!(
        amount_to_mint <= MAX_MINT_AMOUNT,
        CustomError::ExcessiveMintAmount
    );

    let mint_fee = calculate_fee(amount_to_mint, MINT_FEE_BPS)?;
    let adjusted_mint_amount = amount_to_mint - mint_fee;
    
    msg!(
        "Deposit initiated: Collateral={}, Mint={}, Fee={}",
        amount_collateral,
        amount_to_mint,
        mint_fee
    );

    let new_lamport_balance = ctx.accounts.sol_account.lamports() + amount_collateral;
    
    if !ctx.accounts.collateral_account.is_initialized {
        ctx.accounts.collateral_account.is_initialized = true;
        ctx.accounts.collateral_account.depositer = ctx.accounts.depositer.key();
        ctx.accounts.collateral_account.sol_account = ctx.accounts.sol_account.key();
        ctx.accounts.collateral_account.token_account = ctx.accounts.token_account.key();
        ctx.accounts.collateral_account.bump = ctx.bumps.collateral_account;
        ctx.accounts.collateral_account.bump_sol_account = ctx.bumps.sol_account;
        
        msg!("New collateral account initialized for {}", ctx.accounts.depositer.key());
    }
    
    ctx.accounts.collateral_account.lamport_balance = new_lamport_balance;
    ctx.accounts.collateral_account.amount_minted += amount_to_mint;

    check_health_factor(
        amount_collateral,
        &ctx.accounts.collateral_account,
        &ctx.accounts.config_account,
        &ctx.accounts.price_update,
    )?;

    let collateral_value_in_usd = crate::get_usd_value(
        new_lamport_balance, 
        &ctx.accounts.price_update
    )?;
    
    let collateralization_ratio = (collateral_value_in_usd * 100) / ctx.accounts.collateral_account.amount_minted;
    
    require!(
        collateralization_ratio >= MIN_COLLATERAL_RATIO,
        CustomError::InsufficientCollateralization
    );
    
    if collateralization_ratio > MAX_COLLATERAL_RATIO {
        msg!(
            "Warning: High collateralization ratio {}% exceeds recommended maximum {}%",
            collateralization_ratio,
            MAX_COLLATERAL_RATIO
        );
    }

    deposit_sol(
        &ctx.accounts.depositer,
        &ctx.accounts.sol_account,
        amount_collateral,
        &ctx.accounts.system_program,
    )?;

    mint_tokens(
        &ctx.accounts.mint_account,
        &ctx.accounts.token_account,
        &ctx.accounts.token_program,
        adjusted_mint_amount,
        ctx.accounts.config_account.bump_mint_account,
    )?;
    
    if let Some(fee_recipient_account) = &ctx.accounts.fee_recipient {
        msg!(
            "Protocol fee of {} tokens collected to {}",
            mint_fee,
            fee_recipient_account.key()
        );
    }
    
    msg!(
        "Deposit successful: Collateral balance={}, Total minted={}",
        ctx.accounts.collateral_account.lamport_balance,
        ctx.accounts.collateral_account.amount_minted
    );

    Ok(())
}

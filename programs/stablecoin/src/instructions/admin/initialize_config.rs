use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;


use crate::{Config, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT, MINT_DECIMALS, LIQUIDATION_THRESHOLD, LIQUIDATION_BONUS, MIN_HEALTH_FACTOR};


#[derive(Accounts)]
#[instruction()]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
    )]
    pub config_account: Account<'info, Config>,


#[account(
    init,
    payer = authority,
    seeds = [SEED_MINT_ACCOUNT],
    mint::decimals = MINT_DECIMALS,
    mint::authority = mint_account,
    mint::freeze_authority = mint_account,
    mint::token_program = token_program,
    bump,
)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    /// CHECK: This is the token program
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

}



pub fn process_initialize_config(ctx: &mut Context<InitializeConfig>) -> Result<()> {
    *ctx.accounts.config_account = Config {
        authority: ctx.accounts.authority.key(),
        mint_account: ctx.accounts.mint_account.key(),
        liquidation_threshold: LIQUIDATION_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        min_health_factor: MIN_HEALTH_FACTOR,  
        bump: ctx.bumps.config_account,
        bump_mint_account: ctx.bumps.mint_account,
    };

    Ok (())
}
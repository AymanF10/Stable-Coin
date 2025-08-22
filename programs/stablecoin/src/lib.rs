use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;
pub use constants::*;
pub use error::*;

pub mod instructions;
pub mod state;
pub mod constants; 
pub mod error;


declare_id!("GmTUJTroHa7jdWdtVyhuMGQH9YRgcwM5W14JGydfAW1M");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(mut ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(&mut ctx)?;
        Ok(())
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }

    pub fn deposit_collateral_and_mint_tokens(
        ctx: Context<DepositCollateralAndMintTokens>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_tokens(ctx, amount_collateral, amount_to_mint)
    }

    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateralAndBurnTokens>,
        amount_collateral: u64,
        amount_to_burn: u64
    ) -> Result<()> {
        process_redeem_collateral_and_burn_tokens(ctx, amount_collateral, amount_to_burn)
    }

     pub fn liquidate(
        ctx: Context<Liquidate>,
        amount_to_burn: u64
    ) -> Result<()> {
        process_liquidate(ctx, amount_to_burn)
    }
}


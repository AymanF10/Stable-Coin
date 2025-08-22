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
    use instructions::governance::create_proposal::ProposalType;
    use instructions::governance::vote_on_proposal::VoteType;

    // I use this to set up the initial configuration when deploying my stablecoin
    pub fn initialize_config(mut ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(&mut ctx)?;
        Ok(())
    }

    // I allow admins to update key parameters like minimum health factor
    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }

    // This is how users deposit SOL collateral and mint my stablecoin
    pub fn deposit_collateral_and_mint_tokens(
        ctx: Context<DepositCollateralAndMintTokens>,
        amount_collateral: u64,
        amount_to_mint: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_tokens(ctx, amount_collateral, amount_to_mint)
    }

    // Users can burn their stablecoins to get their collateral back
    pub fn redeem_collateral_and_burn_tokens(
        ctx: Context<RedeemCollateralAndBurnTokens>,
        amount_collateral: u64,
        amount_to_burn: u64
    ) -> Result<()> {
        process_redeem_collateral_and_burn_tokens(ctx, amount_collateral, amount_to_burn)
    }

    // If a position becomes undercollateralized, I allow liquidators to step in
    pub fn liquidate(
        ctx: Context<Liquidate>,
        amount_to_burn: u64
    ) -> Result<()> {
        process_liquidate(ctx, amount_to_burn)
    }
    
    // I've added governance so the community can propose changes
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposal_id: u64,
        description: String,
        proposal_type: ProposalType
    ) -> Result<()> {
        process_create_proposal(ctx, proposal_id, description, proposal_type)
    }
    
    // Token holders can vote on governance proposals
    pub fn vote_on_proposal(
        ctx: Context<VoteOnProposal>,
        proposal_id: u64,
        vote_type: VoteType
    ) -> Result<()> {
        process_vote_on_proposal(ctx, proposal_id, vote_type)
    }
    
    // Once a proposal passes, it can be executed to update the protocol
    pub fn execute_proposal(
        ctx: Context<ExecuteProposal>,
        proposal_id: u64
    ) -> Result<()> {
        process_execute_proposal(ctx, proposal_id)
    }
}


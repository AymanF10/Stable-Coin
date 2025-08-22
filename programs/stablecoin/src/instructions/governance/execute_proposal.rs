use anchor_lang::prelude::*;
use crate::{
    Config, SEED_CONFIG_ACCOUNT, SEED_GOVERNANCE, CustomError, 
    EXECUTION_DELAY,
    instructions::governance::create_proposal::{Proposal, ProposalStatus, ProposalType}
};

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub executor: Signer<'info>,
    
    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    
    #[account(
        mut,
        seeds = [SEED_GOVERNANCE, proposal_id.to_le_bytes().as_ref()],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Passed @ CustomError::ProposalVotingEnded,
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub system_program: Program<'info, System>,
}

pub fn process_execute_proposal(
    ctx: Context<ExecuteProposal>,
    _proposal_id: u64,
) -> Result<()> {
    // I need to check the current time to enforce the execution delay
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    
    // I enforce a delay between voting end and execution to give users time to prepare
    require!(
        current_time >= ctx.accounts.proposal.voting_ends_at + EXECUTION_DELAY as i64,
        CustomError::ExecutionDelayNotSatisfied
    );
    
    // I need to check if enough people voted for this proposal
    let total_votes = ctx.accounts.proposal.votes_for + ctx.accounts.proposal.votes_against;
    
    // I require at least some votes to consider a proposal valid
    require!(
        total_votes > 0,
        CustomError::QuorumNotReached
    );
    
    // I only execute proposals that have more votes for than against
    require!(
        ctx.accounts.proposal.votes_for > ctx.accounts.proposal.votes_against,
        CustomError::ProposalVotingEnded
    );
    
    // I handle different types of proposals differently
    match ctx.accounts.proposal.proposal_type {
        ProposalType::UpdateMinHealthFactor(new_value) => {
            // I update the minimum health factor parameter
            ctx.accounts.config.min_health_factor = new_value;
            msg!("Updated min_health_factor to {}", new_value);
        },
        ProposalType::UpdateLiquidationThreshold(new_value) => {
            // I update the liquidation threshold parameter
            ctx.accounts.config.liquidation_threshold = new_value;
            msg!("Updated liquidation_threshold to {}", new_value);
        },
        ProposalType::UpdateLiquidationBonus(new_value) => {
            // I update the liquidation bonus parameter
            ctx.accounts.config.liquidation_bonus = new_value;
            msg!("Updated liquidation_bonus to {}", new_value);
        },
        ProposalType::UpdateOracleConfig(_new_value) => {
            // I'd update oracle configuration in a more complete implementation
            msg!("Oracle config updates not implemented in this version");
        },
        ProposalType::UpdateFeeStructure { mint_fee: _, burn_fee: _, liquidation_fee: _ } => {
            // I'd update fee structure in a more complete implementation
            msg!("Fee structure updates not implemented in this version");
        },
    }
    
    // I mark the proposal as executed so it can't be executed again
    ctx.accounts.proposal.status = ProposalStatus::Executed;
    
    // I log who executed the proposal for transparency
    msg!(
        "Proposal {} executed successfully by {}",
        ctx.accounts.proposal.id,
        ctx.accounts.executor.key()
    );
    
    Ok(())
}

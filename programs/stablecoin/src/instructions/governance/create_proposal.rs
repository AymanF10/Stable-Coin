use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, Token2022};
use crate::{
    Config, SEED_CONFIG_ACCOUNT, SEED_GOVERNANCE, 
    MIN_PROPOSAL_THRESHOLD, VOTING_PERIOD, CustomError
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ProposalType {
    UpdateMinHealthFactor(u64),
    UpdateLiquidationThreshold(u64),
    UpdateLiquidationBonus(u64),
    UpdateOracleConfig(u64),
    UpdateFeeStructure { mint_fee: u16, burn_fee: u16, liquidation_fee: u16 },
}

impl anchor_lang::Space for ProposalType {
    const INIT_SPACE: usize = 32;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum ProposalStatus {
    Active,
    Passed,
    Executed,
    Rejected,
    Cancelled,
}

impl anchor_lang::Space for ProposalStatus {
    const INIT_SPACE: usize = 8;
}

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub id: u64,
    pub proposer: Pubkey,
    pub proposal_type: ProposalType,
    #[max_len(200)]
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub created_at: i64,
    pub voting_ends_at: i64,
    pub status: ProposalStatus,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64, description: String, proposal_type: ProposalType)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    
    #[account(
        mut,
        constraint = proposer_token_account.amount >= MIN_PROPOSAL_THRESHOLD @ CustomError::InsufficientGovernanceBalance,
    )]
    pub proposer_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init,
        payer = proposer,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [SEED_GOVERNANCE, proposal_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

pub fn process_create_proposal(
    ctx: Context<CreateProposal>,
    proposal_id: u64,
    description: String,
    proposal_type: ProposalType,
) -> Result<()> {
    // I need to get the current time to set up the voting period
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    
    // I calculate when voting will end based on my configured voting period
    let voting_ends_at = current_time + VOTING_PERIOD as i64;
    
    // I set up all the proposal details to track it through the governance process
    let proposal = &mut ctx.accounts.proposal;
    proposal.id = proposal_id;
    proposal.proposer = ctx.accounts.proposer.key();
    proposal.proposal_type = proposal_type;
    proposal.description = description;
    
    // Every proposal starts with zero votes
    proposal.votes_for = 0;
    proposal.votes_against = 0;
    
    // I track timing for the proposal lifecycle
    proposal.created_at = current_time;
    proposal.voting_ends_at = voting_ends_at;
    
    // All new proposals start in the Active status
    proposal.status = ProposalStatus::Active;
    proposal.bump = ctx.bumps.proposal;
    
    // I log the proposal creation for transparency
    msg!(
        "Proposal {} created by {} and will end voting at {}",
        proposal_id,
        ctx.accounts.proposer.key(),
        voting_ends_at
    );
    
    Ok(())
}

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenAccount, Token2022};
use crate::{
    SEED_GOVERNANCE, CustomError,
    instructions::governance::create_proposal::{Proposal, ProposalStatus}
};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub enum VoteType {
    For,
    Against,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64, vote_type: VoteType)]
pub struct VoteOnProposal<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    
    #[account(mut)]
    pub voter_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [SEED_GOVERNANCE, proposal_id.to_le_bytes().as_ref()],
        bump = proposal.bump,
        constraint = proposal.status == ProposalStatus::Active @ CustomError::ProposalVotingEnded,
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub token_program: Program<'info, Token2022>,
}

pub fn process_vote_on_proposal(
    ctx: Context<VoteOnProposal>,
    _proposal_id: u64,
    vote_type: VoteType,
) -> Result<()> {
    // I need to check the current time to make sure voting is still open
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    
    // I don't allow voting after the voting period has ended
    require!(
        current_time <= ctx.accounts.proposal.voting_ends_at,
        CustomError::ProposalVotingEnded
    );
    
    // I use the voter's token balance as their voting power - more tokens means more influence
    let voting_power = ctx.accounts.voter_token_account.amount;
    
    // I handle both types of votes differently
    match vote_type {
        VoteType::For => {
            // I add their voting power to the "for" votes, being careful to avoid overflow
            ctx.accounts.proposal.votes_for = ctx.accounts.proposal.votes_for.checked_add(voting_power)
                .ok_or(error!(CustomError::FeeCalculationError))?;
            
            // I log who voted and how much voting power they used
            msg!(
                "Voter {} voted FOR proposal {} with {} voting power",
                ctx.accounts.voter.key(),
                ctx.accounts.proposal.id,
                voting_power
            );
        },
        VoteType::Against => {
            // I add their voting power to the "against" votes, being careful to avoid overflow
            ctx.accounts.proposal.votes_against = ctx.accounts.proposal.votes_against.checked_add(voting_power)
                .ok_or(error!(CustomError::FeeCalculationError))?;
            
            // I log who voted against and how much voting power they used
            msg!(
                "Voter {} voted AGAINST proposal {} with {} voting power",
                ctx.accounts.voter.key(),
                ctx.accounts.proposal.id,
                voting_power
            );
        },
    }
    
    // I log the current vote totals for transparency
    msg!(
        "Current vote totals for proposal {}: FOR={}, AGAINST={}",
        ctx.accounts.proposal.id,
        ctx.accounts.proposal.votes_for,
        ctx.accounts.proposal.votes_against
    );
    
    Ok(())
}

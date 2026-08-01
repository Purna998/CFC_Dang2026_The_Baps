use anchor_lang::prelude::*;
use crate::state::ElectionState;
use crate::errors::VotingError;

#[derive(Accounts)]
#[instruction(election_id: [u8; 16])]
pub struct FinalizeElection<'info> {
    #[account(
        mut,
        seeds = [b"election", election_id.as_ref()],
        bump = election_state.bump,
        constraint = election_state.authority == authority.key() @ VotingError::Unauthorized
    )]
    pub election_state: Account<'info, ElectionState>,

    /// Election authority (must match election_state.authority)
    pub authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<FinalizeElection>,
    election_id: [u8; 16],
) -> Result<()> {
    let election_state = &mut ctx.accounts.election_state;
    let clock = Clock::get()?;

    // Check if already finalized
    require!(!election_state.is_finalized, VotingError::ElectionFinalized);

    // Require at least one vote
    require!(
        election_state.total_votes > 0,
        VotingError::NoVotesInElection
    );

    // Mark as finalized
    election_state.is_finalized = true;

    // Emit event
    emit!(ElectionFinalizedEvent {
        election_id,
        total_votes: election_state.total_votes,
        finalized_at: clock.unix_timestamp,
        finalized_by: ctx.accounts.authority.key(),
    });

    msg!("Election finalized successfully");
    msg!("Election ID: {:?}", election_id);
    msg!("Total votes: {}", election_state.total_votes);
    msg!("Start time: {}", election_state.start_timestamp);
    msg!("End time: {}", election_state.last_vote_timestamp);

    Ok(())
}

#[event]
pub struct ElectionFinalizedEvent {
    pub election_id: [u8; 16],
    pub total_votes: u64,
    pub finalized_at: i64,
    pub finalized_by: Pubkey,
}

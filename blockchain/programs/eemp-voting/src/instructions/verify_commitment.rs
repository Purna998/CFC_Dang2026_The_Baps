use anchor_lang::prelude::*;
use crate::state::VoteCommitment;

#[derive(Accounts)]
#[instruction(vote_hash: [u8; 32])]
pub struct VerifyCommitment<'info> {
    #[account(
        seeds = [
            b"vote_commitment",
            vote_commitment.election_id.as_ref(),
            vote_hash.as_ref()
        ],
        bump = vote_commitment.bump
    )]
    pub vote_commitment: Account<'info, VoteCommitment>,
}

pub fn handler(
    ctx: Context<VerifyCommitment>,
    vote_hash: [u8; 32],
) -> Result<()> {
    let commitment = &ctx.accounts.vote_commitment;

    // Verify vote hash matches
    require!(
        commitment.vote_hash == vote_hash,
        crate::errors::VotingError::InvalidVoteHash
    );

    // Emit verification event
    emit!(VoteVerifiedEvent {
        election_id: commitment.election_id,
        vote_hash: commitment.vote_hash,
        timestamp: commitment.timestamp,
        sequence_number: commitment.sequence_number,
        verified_at: Clock::get()?.unix_timestamp,
    });

    msg!("Vote commitment verified");
    msg!("Election ID: {:?}", commitment.election_id);
    msg!("Vote Hash: {:?}", commitment.vote_hash);
    msg!("Original timestamp: {}", commitment.timestamp);
    msg!("Sequence number: {}", commitment.sequence_number);

    Ok(())
}

#[event]
pub struct VoteVerifiedEvent {
    pub election_id: [u8; 16],
    pub vote_hash: [u8; 32],
    pub timestamp: i64,
    pub sequence_number: u64,
    pub verified_at: i64,
}

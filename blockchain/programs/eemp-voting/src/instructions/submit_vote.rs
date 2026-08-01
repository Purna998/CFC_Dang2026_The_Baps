use anchor_lang::prelude::*;
use crate::state::{ProgramState, ElectionState, VoteCommitment};
use crate::errors::VotingError;

#[derive(Accounts)]
#[instruction(election_id: [u8; 16], vote_hash: [u8; 32])]
pub struct SubmitVoteCommitment<'info> {
    #[account(
        seeds = [b"program_state"],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(
        init_if_needed,
        payer = payer,
        space = ElectionState::LEN,
        seeds = [b"election", election_id.as_ref()],
        bump
    )]
    pub election_state: Account<'info, ElectionState>,

    #[account(
        init,
        payer = payer,
        space = VoteCommitment::LEN,
        seeds = [
            b"vote_commitment",
            election_id.as_ref(),
            vote_hash.as_ref()
        ],
        bump
    )]
    pub vote_commitment: Account<'info, VoteCommitment>,

    /// Authority submitting the vote (backend service)
    pub authority: Signer<'info>,

    /// Payer for the transaction
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<SubmitVoteCommitment>,
    election_id: [u8; 16],
    vote_hash: [u8; 32],
    commitment: [u8; 64],
    metadata: Option<Vec<u8>>,
) -> Result<()> {
    let program_state = &ctx.accounts.program_state;
    let election_state = &mut ctx.accounts.election_state;
    let vote_commitment = &mut ctx.accounts.vote_commitment;
    let clock = Clock::get()?;

    // Check if program is paused
    require!(!program_state.is_paused, VotingError::ProgramPaused);

    // Check if election is finalized
    require!(!election_state.is_finalized, VotingError::ElectionFinalized);

    // Validate metadata size
    VoteCommitment::validate_metadata(&metadata)?;

    // Validate vote hash (must not be all zeros)
    require!(
        vote_hash != [0u8; 32],
        VotingError::InvalidVoteHash
    );

    // Validate commitment signature (must not be all zeros)
    require!(
        commitment != [0u8; 64],
        VotingError::InvalidCommitment
    );

    // Initialize election state if this is the first vote
    if election_state.total_votes == 0 {
        election_state.election_id = election_id;
        election_state.tenant_id = [0u8; 16]; // Set by backend
        election_state.authority = ctx.accounts.authority.key();
        election_state.total_votes = 0;
        election_state.start_timestamp = clock.unix_timestamp;
        election_state.last_vote_timestamp = clock.unix_timestamp;
        election_state.is_finalized = false;
        election_state.merkle_root = None;
        election_state.created_at = clock.unix_timestamp;
        election_state.bump = ctx.bumps.election_state;
    }

    // Increment vote count
    election_state.total_votes = election_state
        .total_votes
        .checked_add(1)
        .ok_or(VotingError::ArithmeticOverflow)?;

    election_state.last_vote_timestamp = clock.unix_timestamp;

    // Store vote commitment
    vote_commitment.election_id = election_id;
    vote_commitment.vote_hash = vote_hash;
    vote_commitment.commitment = commitment;
    vote_commitment.timestamp = clock.unix_timestamp;
    vote_commitment.slot = clock.slot;
    vote_commitment.sequence_number = election_state.total_votes;
    vote_commitment.metadata = metadata.clone();
    vote_commitment.bump = ctx.bumps.vote_commitment;

    // Emit event for indexers
    emit!(VoteCommittedEvent {
        election_id,
        vote_hash,
        sequence_number: vote_commitment.sequence_number,
        timestamp: vote_commitment.timestamp,
        slot: vote_commitment.slot,
    });

    msg!("Vote commitment submitted successfully");
    msg!("Election ID: {:?}", election_id);
    msg!("Vote Hash: {:?}", vote_hash);
    msg!("Sequence: {}", vote_commitment.sequence_number);
    msg!("Total votes in election: {}", election_state.total_votes);

    Ok(())
}

#[event]
pub struct VoteCommittedEvent {
    pub election_id: [u8; 16],
    pub vote_hash: [u8; 32],
    pub sequence_number: u64,
    pub timestamp: i64,
    pub slot: u64,
}

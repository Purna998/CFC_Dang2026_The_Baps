use anchor_lang::prelude::*;
use crate::state::{ElectionState, MerkleProof};
use crate::errors::VotingError;

#[derive(Accounts)]
#[instruction(election_id: [u8; 16])]
pub struct GenerateMerkleRoot<'info> {
    #[account(
        mut,
        seeds = [b"election", election_id.as_ref()],
        bump = election_state.bump,
        constraint = election_state.is_finalized @ VotingError::ElectionNotFinalized,
        constraint = election_state.authority == authority.key() @ VotingError::Unauthorized
    )]
    pub election_state: Account<'info, ElectionState>,

    #[account(
        init,
        payer = payer,
        space = MerkleProof::LEN,
        seeds = [b"merkle_proof", election_id.as_ref()],
        bump
    )]
    pub merkle_proof: Account<'info, MerkleProof>,

    /// Election authority
    pub authority: Signer<'info>,

    /// Payer for storage
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<GenerateMerkleRoot>,
    election_id: [u8; 16],
) -> Result<()> {
    let election_state = &mut ctx.accounts.election_state;
    let merkle_proof = &mut ctx.accounts.merkle_proof;
    let clock = Clock::get()?;

    // Check if merkle root already exists
    require!(
        election_state.merkle_root.is_none(),
        VotingError::MerkleRootExists
    );

    // Require at least one vote
    require!(
        election_state.total_votes > 0,
        VotingError::NoVotesInElection
    );

    // In production, this would iterate through all vote commitments
    // and compute the actual merkle tree root.
    // For this implementation, we'll store a placeholder that gets
    // computed by the backend indexer.

    // Placeholder root (in production, compute from all vote hashes)
    let placeholder_root = compute_placeholder_root(
        election_id,
        election_state.total_votes,
    );

    // Store merkle proof
    merkle_proof.election_id = election_id;
    merkle_proof.root = placeholder_root;
    merkle_proof.total_leaves = election_state.total_votes;
    merkle_proof.generated_at = clock.unix_timestamp;
    merkle_proof.generated_by = ctx.accounts.authority.key();

    // Update election state
    election_state.merkle_root = Some(placeholder_root);

    // Emit event
    emit!(MerkleRootGeneratedEvent {
        election_id,
        root: placeholder_root,
        total_votes: election_state.total_votes,
        generated_at: clock.unix_timestamp,
    });

    msg!("Merkle root generated");
    msg!("Election ID: {:?}", election_id);
    msg!("Root: {:?}", placeholder_root);
    msg!("Total votes: {}", election_state.total_votes);

    Ok(())
}

/// Compute a deterministic placeholder root
/// In production, this would be replaced with actual merkle tree computation
fn compute_placeholder_root(election_id: [u8; 16], total_votes: u64) -> [u8; 32] {
    use anchor_lang::solana_program::hash::{hash, hashv};

    let hash_result = hashv(&[
        b"merkle_root",
        election_id.as_ref(),
        &total_votes.to_le_bytes(),
    ]);

    hash_result.to_bytes()
}

#[event]
pub struct MerkleRootGeneratedEvent {
    pub election_id: [u8; 16],
    pub root: [u8; 32],
    pub total_votes: u64,
    pub generated_at: i64,
}

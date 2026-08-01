use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;

declare_id!("EEMPvot1ng11111111111111111111111111111111");

#[program]
pub mod eemp_voting {
    use super::*;

    /// Initialize the vote commitment program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    /// Submit a vote commitment to the blockchain
    ///
    /// This stores only the cryptographic commitment (hash) of the vote,
    /// never the actual vote content or voter identity.
    ///
    /// # Arguments
    /// * `election_id` - UUID of the election (as bytes)
    /// * `vote_hash` - SHA-256 hash of the encrypted ballot
    /// * `commitment` - Ed25519 signature of the vote
    /// * `metadata` - Optional metadata (max 128 bytes)
    pub fn submit_vote_commitment(
        ctx: Context<SubmitVoteCommitment>,
        election_id: [u8; 16],
        vote_hash: [u8; 32],
        commitment: [u8; 64],
        metadata: Option<Vec<u8>>,
    ) -> Result<()> {
        instructions::submit_vote::handler(ctx, election_id, vote_hash, commitment, metadata)
    }

    /// Verify a vote commitment exists and is valid
    ///
    /// Returns commitment data for verification purposes
    pub fn verify_commitment(
        ctx: Context<VerifyCommitment>,
        vote_hash: [u8; 32],
    ) -> Result<()> {
        instructions::verify_commitment::handler(ctx, vote_hash)
    }

    /// Finalize an election (prevents further vote submissions)
    ///
    /// Only the election authority can finalize
    pub fn finalize_election(
        ctx: Context<FinalizeElection>,
        election_id: [u8; 16],
    ) -> Result<()> {
        instructions::finalize_election::handler(ctx, election_id)
    }

    /// Generate a merkle root for all commitments in an election
    ///
    /// Used for batch verification and audit purposes
    pub fn generate_merkle_root(
        ctx: Context<GenerateMerkleRoot>,
        election_id: [u8; 16],
    ) -> Result<()> {
        instructions::generate_merkle_root::handler(ctx, election_id)
    }
}

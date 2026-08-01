use anchor_lang::prelude::*;

/// Global program state
#[account]
pub struct ProgramState {
    /// Program authority (can update settings)
    pub authority: Pubkey,
    /// Total number of commitments submitted
    pub total_commitments: u64,
    /// Program version
    pub version: u8,
    /// Emergency pause flag
    pub is_paused: bool,
    /// Reserved for future use
    pub reserved: [u8; 64],
}

impl ProgramState {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        8 +  // total_commitments
        1 +  // version
        1 +  // is_paused
        64;  // reserved
}

/// Election state on blockchain
#[account]
pub struct ElectionState {
    /// Election ID (UUID from backend)
    pub election_id: [u8; 16],
    /// Organization/tenant ID
    pub tenant_id: [u8; 16],
    /// Election authority (who can finalize)
    pub authority: Pubkey,
    /// Total votes committed
    pub total_votes: u64,
    /// Timestamp of first vote
    pub start_timestamp: i64,
    /// Timestamp of last vote
    pub last_vote_timestamp: i64,
    /// Election finalized flag
    pub is_finalized: bool,
    /// Merkle root of all commitments (set after finalization)
    pub merkle_root: Option<[u8; 32]>,
    /// Creation timestamp
    pub created_at: i64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl ElectionState {
    pub const LEN: usize = 8 + // discriminator
        16 +  // election_id
        16 +  // tenant_id
        32 +  // authority
        8 +   // total_votes
        8 +   // start_timestamp
        8 +   // last_vote_timestamp
        1 +   // is_finalized
        1 + 32 + // Option<merkle_root>
        8 +   // created_at
        1;    // bump
}

/// Individual vote commitment record
///
/// This is the immutable proof stored on-chain.
/// It contains NO voter identity or vote content.
#[account]
pub struct VoteCommitment {
    /// Election this vote belongs to
    pub election_id: [u8; 16],
    /// SHA-256 hash of the encrypted ballot
    pub vote_hash: [u8; 32],
    /// Ed25519 signature commitment
    pub commitment: [u8; 64],
    /// Timestamp when commitment was submitted
    pub timestamp: i64,
    /// Slot number for ordering
    pub slot: u64,
    /// Sequential number within election
    pub sequence_number: u64,
    /// Optional metadata (max 128 bytes)
    /// Can store things like: client version, submission method
    pub metadata: Option<Vec<u8>>,
    /// Bump seed for PDA
    pub bump: u8,
}

impl VoteCommitment {
    pub const MAX_METADATA_LEN: usize = 128;

    pub const LEN: usize = 8 + // discriminator
        16 +  // election_id
        32 +  // vote_hash
        64 +  // commitment
        8 +   // timestamp
        8 +   // slot
        8 +   // sequence_number
        4 + Self::MAX_METADATA_LEN + // Option<Vec> metadata
        1;    // bump

    /// Validate metadata size
    pub fn validate_metadata(metadata: &Option<Vec<u8>>) -> Result<()> {
        if let Some(data) = metadata {
            require!(
                data.len() <= Self::MAX_METADATA_LEN,
                crate::errors::VotingError::MetadataTooLarge
            );
        }
        Ok(())
    }
}

/// Merkle proof for vote verification
#[account]
pub struct MerkleProof {
    /// Election ID
    pub election_id: [u8; 16],
    /// Merkle root hash
    pub root: [u8; 32],
    /// Total leaves in the tree
    pub total_leaves: u64,
    /// Timestamp when generated
    pub generated_at: i64,
    /// Authority who generated
    pub generated_by: Pubkey,
}

impl MerkleProof {
    pub const LEN: usize = 8 + // discriminator
        16 + // election_id
        32 + // root
        8 +  // total_leaves
        8 +  // generated_at
        32;  // generated_by
}

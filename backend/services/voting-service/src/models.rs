//! Voting domain models

use chrono::{DateTime, Utc};
use eemp_domain::{ElectionId, TenantId, UserId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Ballot entity (stored encrypted in database)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ballot {
    pub ballot_id: Uuid,
    pub tenant_id: TenantId,
    pub election_id: ElectionId,
    pub voter_id: UserId,
    pub encrypted_ballot: String, // Base64 encoded encrypted data
    pub encryption_key_id: Uuid,
    pub ballot_hash: String, // SHA-256 hash
    pub voter_receipt_code: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub cast_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
}

/// Ballot content (plaintext, before encryption)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BallotContent {
    pub election_id: String,
    pub votes: Vec<Vote>,
}

/// Individual vote for a position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub position_id: String,
    pub candidate_ids: Vec<String>, // Can be multiple for panel elections
    pub is_abstain: bool,
}

/// Vote commitment for blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteCommitment {
    pub commitment_id: Uuid,
    pub tenant_id: TenantId,
    pub election_id: ElectionId,
    pub ballot_id: Uuid,
    pub commitment_hash: String,
    pub blockchain_transaction_id: Option<String>,
    pub blockchain_network: String,
    pub blockchain_block_height: Option<i64>,
    pub blockchain_timestamp: Option<DateTime<Utc>>,
    pub signature: String,
    pub submitted_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
}

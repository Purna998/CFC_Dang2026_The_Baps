//! Result domain models

use chrono::{DateTime, Utc};
use eemp_domain::{ElectionId, TenantId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Election result for a candidate in a position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Result {
    pub result_id: Uuid,
    pub tenant_id: TenantId,
    pub election_id: ElectionId,
    pub position_id: Uuid,
    pub candidate_id: Option<Uuid>, // None for abstain/invalid votes
    pub vote_count: i32,
    pub vote_percentage: Option<f64>,
    pub is_winner: bool,
    pub rank: Option<i32>,
    pub calculated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Decrypted ballot content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptedBallot {
    pub ballot_id: Uuid,
    pub election_id: String,
    pub votes: Vec<DecryptedVote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptedVote {
    pub position_id: String,
    pub candidate_ids: Vec<String>,
    pub is_abstain: bool,
}

/// Vote tally for a position
#[derive(Debug, Clone)]
pub struct PositionTally {
    pub position_id: Uuid,
    pub candidate_votes: std::collections::HashMap<Uuid, i32>,
    pub abstain_count: i32,
    pub total_votes: i32,
}

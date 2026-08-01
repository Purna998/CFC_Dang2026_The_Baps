//! Data Transfer Objects for Voting Service

use eemp_domain::{ElectionId, UserId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CastBallotRequest {
    pub election_id: ElectionId,
    pub votes: Vec<VoteDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VoteDto {
    pub position_id: Uuid,
    pub candidate_ids: Vec<Uuid>,
    #[serde(default)]
    pub is_abstain: bool,
}

#[derive(Debug, Serialize)]
pub struct CastBallotResponse {
    pub ballot_id: Uuid,
    pub receipt_code: String,
    pub ballot_hash: String,
    pub cast_at: String,
    pub commitment_created: bool,
}

#[derive(Debug, Deserialize)]
pub struct VerifyReceiptRequest {
    pub receipt_code: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyReceiptResponse {
    pub valid: bool,
    pub ballot_id: Option<Uuid>,
    pub election_id: Option<ElectionId>,
    pub cast_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VotingStatusResponse {
    pub election_id: ElectionId,
    pub has_voted: bool,
    pub ballot_id: Option<Uuid>,
    pub cast_at: Option<String>,
}

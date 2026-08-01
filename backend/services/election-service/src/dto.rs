//! Data Transfer Objects for Election Service

use chrono::{DateTime, Utc};
use eemp_domain::{ElectionId, ElectionStatus, ElectionType, TenantId, UserId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::models::{Candidate, Election, EligibilityRuleType, Position};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateElectionRequest {
    #[validate(length(min = 3, max = 200))]
    pub title: String,
    pub description: Option<String>,
    pub election_type: ElectionType,
    pub voting_start_time: DateTime<Utc>,
    pub voting_end_time: DateTime<Utc>,
    pub result_publish_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub allow_write_in_candidates: bool,
    #[serde(default = "default_true")]
    pub allow_abstain: bool,
    #[serde(default = "default_true")]
    pub require_identity_verification: bool,
    #[serde(default = "default_true")]
    pub enable_blockchain_verification: bool,
    pub max_votes_per_voter: Option<i32>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize)]
pub struct CreateElectionResponse {
    pub election_id: ElectionId,
    pub title: String,
    pub status: ElectionStatus,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateElectionRequest {
    #[validate(length(min = 3, max = 200))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub voting_start_time: Option<DateTime<Utc>>,
    pub voting_end_time: Option<DateTime<Utc>>,
    pub result_publish_time: Option<DateTime<Utc>>,
    pub allow_write_in_candidates: Option<bool>,
    pub allow_abstain: Option<bool>,
    pub require_identity_verification: Option<bool>,
    pub enable_blockchain_verification: Option<bool>,
    pub max_votes_per_voter: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ElectionResponse {
    pub election_id: ElectionId,
    pub tenant_id: TenantId,
    pub title: String,
    pub description: Option<String>,
    pub election_type: ElectionType,
    pub status: ElectionStatus,
    pub voting_start_time: String,
    pub voting_end_time: String,
    pub result_publish_time: Option<String>,
    pub allow_write_in_candidates: bool,
    pub allow_abstain: bool,
    pub require_identity_verification: bool,
    pub enable_blockchain_verification: bool,
    pub max_votes_per_voter: Option<i32>,
    pub created_by: UserId,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: Option<String>,
    pub archived_at: Option<String>,
}

impl From<Election> for ElectionResponse {
    fn from(election: Election) -> Self {
        Self {
            election_id: election.election_id,
            tenant_id: election.tenant_id,
            title: election.title,
            description: election.description,
            election_type: election.election_type,
            status: election.status,
            voting_start_time: election.voting_start_time.to_rfc3339(),
            voting_end_time: election.voting_end_time.to_rfc3339(),
            result_publish_time: election.result_publish_time.map(|dt| dt.to_rfc3339()),
            allow_write_in_candidates: election.allow_write_in_candidates,
            allow_abstain: election.allow_abstain,
            require_identity_verification: election.require_identity_verification,
            enable_blockchain_verification: election.enable_blockchain_verification,
            max_votes_per_voter: election.max_votes_per_voter,
            created_by: election.created_by,
            created_at: election.created_at.to_rfc3339(),
            updated_at: election.updated_at.to_rfc3339(),
            published_at: election.published_at.map(|dt| dt.to_rfc3339()),
            archived_at: election.archived_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TransitionElectionRequest {
    pub new_status: ElectionStatus,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePositionRequest {
    #[validate(length(min = 2, max = 200))]
    pub title: String,
    pub description: Option<String>,
    #[validate(range(min = 0))]
    pub display_order: i32,
    #[validate(range(min = 1))]
    pub seats_available: i32,
    pub min_votes_required: Option<i32>,
    pub max_votes_per_voter: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PositionResponse {
    pub position_id: Uuid,
    pub election_id: ElectionId,
    pub title: String,
    pub description: Option<String>,
    pub display_order: i32,
    pub seats_available: i32,
    pub min_votes_required: Option<i32>,
    pub max_votes_per_voter: Option<i32>,
}

impl From<Position> for PositionResponse {
    fn from(position: Position) -> Self {
        Self {
            position_id: position.position_id,
            election_id: position.election_id,
            title: position.title,
            description: position.description,
            display_order: position.display_order,
            seats_available: position.seats_available,
            min_votes_required: position.min_votes_required,
            max_votes_per_voter: position.max_votes_per_voter,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCandidateRequest {
    pub user_id: Option<UserId>,
    #[validate(length(min = 2, max = 100))]
    pub full_name: String,
    pub bio: Option<String>,
    #[validate(url)]
    pub photo_url: Option<String>,
    #[validate(url)]
    pub manifesto_url: Option<String>,
    #[validate(range(min = 0))]
    pub display_order: i32,
    #[serde(default)]
    pub is_write_in: bool,
}

#[derive(Debug, Serialize)]
pub struct CandidateResponse {
    pub candidate_id: Uuid,
    pub election_id: ElectionId,
    pub position_id: Uuid,
    pub user_id: Option<UserId>,
    pub full_name: String,
    pub bio: Option<String>,
    pub photo_url: Option<String>,
    pub manifesto_url: Option<String>,
    pub display_order: i32,
    pub is_approved: bool,
    pub is_write_in: bool,
    pub withdrawn_at: Option<String>,
}

impl From<Candidate> for CandidateResponse {
    fn from(candidate: Candidate) -> Self {
        Self {
            candidate_id: candidate.candidate_id,
            election_id: candidate.election_id,
            position_id: candidate.position_id,
            user_id: candidate.user_id,
            full_name: candidate.full_name,
            bio: candidate.bio,
            photo_url: candidate.photo_url,
            manifesto_url: candidate.manifesto_url,
            display_order: candidate.display_order,
            is_approved: candidate.is_approved,
            is_write_in: candidate.is_write_in,
            withdrawn_at: candidate.withdrawn_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ApproveCandidateRequest {
    pub approved: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateEligibilityRuleRequest {
    pub rule_type: EligibilityRuleType,
    pub rule_config: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ElectionListResponse {
    pub elections: Vec<ElectionResponse>,
    pub total: i64,
}

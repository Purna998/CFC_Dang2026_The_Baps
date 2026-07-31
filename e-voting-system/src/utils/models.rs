use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// User models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Voter,
    Party,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::Voter => write!(f, "voter"),
            UserRole::Party => write!(f, "party"),
        }
    }
}

// Voter models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Voter {
    pub id: Uuid,
    pub user_id: Uuid,
    pub voter_id: String,
    pub national_id: String,
    pub date_of_birth: NaiveDate,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Nepal-specific fields
    pub citizenship_number: Option<String>,
    pub citizenship_issue_district: Option<String>,
    pub province: Option<String>,
    pub district: Option<String>,
    pub municipality: Option<String>,
    pub ward_number: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterVoterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 2))]
    pub full_name: String,
    pub voter_id: String,
    pub national_id: String,
    pub date_of_birth: NaiveDate,
    pub address: Option<String>,
    pub phone: Option<String>,
    // Nepal-specific fields
    pub citizenship_number: Option<String>,
    pub citizenship_issue_district: Option<String>,
    pub province: Option<String>,
    pub district: Option<String>,
    pub municipality: Option<String>,
    pub ward_number: Option<i32>,
}

// Election models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Election {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub election_type: String,
    pub status: ElectionStatus,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Nepal-specific fields
    pub election_level: Option<String>,
    pub province: Option<String>,
    pub district: Option<String>,
    pub municipality: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ElectionStatus {
    Draft,
    Open,
    Closed,
    Archived,
}

impl std::fmt::Display for ElectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElectionStatus::Draft => write!(f, "draft"),
            ElectionStatus::Open => write!(f, "open"),
            ElectionStatus::Closed => write!(f, "closed"),
            ElectionStatus::Archived => write!(f, "archived"),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateElectionRequest {
    #[validate(length(min = 3))]
    pub title: String,
    pub description: Option<String>,
    pub election_type: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    // Nepal-specific fields
    pub election_level: Option<String>,
    pub province: Option<String>,
    pub district: Option<String>,
    pub municipality: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateElectionRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub election_type: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

// Party models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Party {
    pub id: Uuid,
    pub name: String,
    pub abbreviation: Option<String>,
    pub symbol_url: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    // Nepal-specific fields
    pub registration_number: Option<String>,
    pub party_color: Option<String>,
    pub election_symbol_name: Option<String>,
    // Party account fields
    pub user_id: Option<Uuid>,
    pub is_verified: bool,
    pub updated_at: DateTime<Utc>,
    pub website: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub headquarters_address: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterPartyRequest {
    #[validate(email)]
    pub email: String,
    // Password will be auto-generated
    #[validate(length(min = 2))]
    pub name: String,
    pub abbreviation: Option<String>,
    pub registration_number: String,
    pub description: Option<String>,
    pub party_color: Option<String>,
    pub election_symbol_name: Option<String>,
    pub website: Option<String>,
    pub contact_phone: Option<String>,
    pub headquarters_address: Option<String>,
}

// Candidate models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Candidate {
    pub id: Uuid,
    pub election_id: Uuid,
    pub party_id: Option<Uuid>,
    pub full_name: String,
    pub photo_url: Option<String>,
    pub biography: Option<String>,
    pub position_number: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Nepal-specific fields
    pub age: Option<i32>,
    pub education: Option<String>,
    pub constituency: Option<String>,
    // FPTP/PR/Independent fields
    pub candidate_type: String, // 'fptp', 'pr', 'independent'
    pub is_verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<Uuid>,
    pub pr_list_position: Option<i32>,
    pub constituency_number: Option<String>,
    pub is_independent: bool,
    pub independent_symbol_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCandidateRequest {
    pub election_id: Uuid,
    pub party_id: Option<Uuid>,
    #[validate(length(min = 2))]
    pub full_name: String,
    pub photo_url: Option<String>,
    pub biography: Option<String>,
    pub position_number: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCandidateRequest {
    pub party_id: Option<Uuid>,
    pub full_name: Option<String>,
    pub photo_url: Option<String>,
    pub biography: Option<String>,
    pub position_number: Option<i32>,
}

// Ballot models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ballot {
    pub id: Uuid,
    pub election_id: Uuid,
    pub candidate_id: Uuid,
    pub encrypted_voter_hash: String,
    pub cast_at: DateTime<Utc>,
    pub verification_code: String,
}

#[derive(Debug, Deserialize)]
pub struct CastVoteRequest {
    pub election_id: Uuid,
    pub candidate_id: Uuid,
}

// Voter participation
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VoterParticipation {
    pub id: Uuid,
    pub election_id: Uuid,
    pub voter_id: Uuid,
    pub has_voted: bool,
    pub voted_at: DateTime<Utc>,
}

// Audit log
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Election results
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ElectionResult {
    pub election_id: Uuid,
    pub election_title: String,
    pub status: ElectionStatus,
    pub candidate_id: Option<Uuid>,
    pub candidate_name: Option<String>,
    pub position_number: Option<i32>,
    pub party_name: Option<String>,
    pub vote_count: Option<i64>,
    pub vote_percentage: Option<f64>,
}

// Auth models
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub role: UserRole,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    #[validate(length(min = 8))]
    pub new_password: String,
}

// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub email: String,
    pub role: String,
    pub exp: i64,
}

// Constituency models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Constituency {
    pub id: Uuid,
    pub constituency_number: String,
    pub constituency_name: String,
    pub province: String,
    pub district: String,
    pub total_voters: i32,
    pub election_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateConstituencyRequest {
    pub constituency_number: String,
    pub constituency_name: String,
    pub province: String,
    pub district: String,
    pub election_id: Option<Uuid>,
}

// PR Candidate List models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PrCandidateList {
    pub id: Uuid,
    pub party_id: Uuid,
    pub election_id: Uuid,
    pub candidate_id: Uuid,
    pub list_position: i32,
    pub created_at: DateTime<Utc>,
}

// FPTP Nomination models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FptpNomination {
    pub id: Uuid,
    pub candidate_id: Uuid,
    pub constituency_id: Uuid,
    pub party_id: Option<Uuid>,
    pub is_independent: bool,
    pub nomination_status: String, // 'pending', 'approved', 'rejected'
    pub created_at: DateTime<Utc>,
}

// Candidate Verification Request models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CandidateVerificationRequest {
    pub id: Uuid,
    pub candidate_id: Uuid,
    pub requested_by: Uuid,
    pub request_type: String, // 'self', 'party'
    pub citizenship_proof_url: Option<String>,
    pub educational_proof_url: Option<String>,
    pub party_nomination_url: Option<String>,
    pub status: String, // 'pending', 'approved', 'rejected'
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Response wrappers
#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

// Party creation response with temporary password
#[derive(Debug, Serialize)]
pub struct PartyCreationResponse {
    pub party: Party,
    pub temporary_password: String,
    pub message: String,
}

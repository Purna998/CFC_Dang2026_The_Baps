//! Election domain models

use chrono::{DateTime, Utc};
use eemp_domain::{ElectionId, ElectionStatus, ElectionType, TenantId, UserId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Election entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Election {
    pub election_id: ElectionId,
    pub tenant_id: TenantId,
    pub title: String,
    pub description: Option<String>,
    pub election_type: ElectionType,
    pub status: ElectionStatus,
    pub voting_start_time: DateTime<Utc>,
    pub voting_end_time: DateTime<Utc>,
    pub result_publish_time: Option<DateTime<Utc>>,
    pub allow_write_in_candidates: bool,
    pub allow_abstain: bool,
    pub require_identity_verification: bool,
    pub enable_blockchain_verification: bool,
    pub max_votes_per_voter: Option<i32>,
    pub created_by: UserId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub archived_at: Option<DateTime<Utc>>,
}

/// Position entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub position_id: Uuid,
    pub tenant_id: TenantId,
    pub election_id: ElectionId,
    pub title: String,
    pub description: Option<String>,
    pub display_order: i32,
    pub seats_available: i32,
    pub min_votes_required: Option<i32>,
    pub max_votes_per_voter: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Candidate entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub candidate_id: Uuid,
    pub tenant_id: TenantId,
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
    pub approved_by: Option<UserId>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub withdrawn_at: Option<DateTime<Utc>>,
}

/// Eligibility rule entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EligibilityRule {
    pub rule_id: Uuid,
    pub tenant_id: TenantId,
    pub election_id: ElectionId,
    pub rule_type: EligibilityRuleType,
    pub rule_config: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Eligibility rule types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EligibilityRuleType {
    AllUsers,
    RoleBasedAccess,
    DepartmentBased,
    CustomList,
    MinimumTenure,
}

impl std::fmt::Display for EligibilityRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AllUsers => write!(f, "AllUsers"),
            Self::RoleBasedAccess => write!(f, "RoleBasedAccess"),
            Self::DepartmentBased => write!(f, "DepartmentBased"),
            Self::CustomList => write!(f, "CustomList"),
            Self::MinimumTenure => write!(f, "MinimumTenure"),
        }
    }
}

impl std::str::FromStr for EligibilityRuleType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AllUsers" => Ok(Self::AllUsers),
            "RoleBasedAccess" => Ok(Self::RoleBasedAccess),
            "DepartmentBased" => Ok(Self::DepartmentBased),
            "CustomList" => Ok(Self::CustomList),
            "MinimumTenure" => Ok(Self::MinimumTenure),
            _ => Err(format!("Invalid eligibility rule type: {}", s)),
        }
    }
}

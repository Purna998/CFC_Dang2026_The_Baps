//! Domain enums

use serde::{Deserialize, Serialize};

/// User roles (RBAC)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    OrganizationOwner,
    OrganizationAdmin,
    ElectionManager,
    ElectionOfficer,
    Voter,
    Candidate,
    Auditor,
    Observer,
}

/// Election status (state machine)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ElectionStatus {
    Draft,
    Review,
    Scheduled,
    Open,
    Closed,
    Verifying,
    Published,
    Archived,
}

/// Election type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ElectionType {
    Individual,
    PostWise,
    Panel,
    RankedChoice,
}

//! Domain enums

use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

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

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OrganizationOwner => write!(f, "OrganizationOwner"),
            Self::OrganizationAdmin => write!(f, "OrganizationAdmin"),
            Self::ElectionManager => write!(f, "ElectionManager"),
            Self::ElectionOfficer => write!(f, "ElectionOfficer"),
            Self::Voter => write!(f, "Voter"),
            Self::Candidate => write!(f, "Candidate"),
            Self::Auditor => write!(f, "Auditor"),
            Self::Observer => write!(f, "Observer"),
        }
    }
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OrganizationOwner" => Ok(Self::OrganizationOwner),
            "OrganizationAdmin" => Ok(Self::OrganizationAdmin),
            "ElectionManager" => Ok(Self::ElectionManager),
            "ElectionOfficer" => Ok(Self::ElectionOfficer),
            "Voter" => Ok(Self::Voter),
            "Candidate" => Ok(Self::Candidate),
            "Auditor" => Ok(Self::Auditor),
            "Observer" => Ok(Self::Observer),
            _ => Err(format!("Invalid user role: {}", s)),
        }
    }
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

//! Organization domain models

use chrono::{DateTime, Utc};
use eemp_domain::TenantId;
use serde::{Deserialize, Serialize};

/// Organization entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub tenant_id: TenantId,
    pub name: String,
    pub organization_type: OrganizationType,
    pub domain: Option<String>,
    pub subdomain: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: String,
    pub postal_code: Option<String>,
    pub settings: OrganizationSettings,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Organization type (per BRD requirements)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationType {
    University,
    College,
    School,
    Company,
    Municipality,
    Ngo,
    Ingo,
    Hospital,
    Cooperative,
    Club,
    Community,
    Association,
    TradeUnion,
    ReligiousOrganization,
}

impl std::fmt::Display for OrganizationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::University => write!(f, "University"),
            Self::College => write!(f, "College"),
            Self::School => write!(f, "School"),
            Self::Company => write!(f, "Company"),
            Self::Municipality => write!(f, "Municipality"),
            Self::Ngo => write!(f, "NGO"),
            Self::Ingo => write!(f, "INGO"),
            Self::Hospital => write!(f, "Hospital"),
            Self::Cooperative => write!(f, "Cooperative"),
            Self::Club => write!(f, "Club"),
            Self::Community => write!(f, "Community"),
            Self::Association => write!(f, "Association"),
            Self::TradeUnion => write!(f, "TradeUnion"),
            Self::ReligiousOrganization => write!(f, "ReligiousOrganization"),
        }
    }
}

impl std::str::FromStr for OrganizationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "University" => Ok(Self::University),
            "College" => Ok(Self::College),
            "School" => Ok(Self::School),
            "Company" => Ok(Self::Company),
            "Municipality" => Ok(Self::Municipality),
            "NGO" => Ok(Self::Ngo),
            "INGO" => Ok(Self::Ingo),
            "Hospital" => Ok(Self::Hospital),
            "Cooperative" => Ok(Self::Cooperative),
            "Club" => Ok(Self::Club),
            "Community" => Ok(Self::Community),
            "Association" => Ok(Self::Association),
            "TradeUnion" => Ok(Self::TradeUnion),
            "ReligiousOrganization" => Ok(Self::ReligiousOrganization),
            _ => Err(format!("Invalid organization type: {}", s)),
        }
    }
}

/// Organization settings (JSON stored in database)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSettings {
    pub allow_public_registration: bool,
    pub require_email_verification: bool,
    pub require_admin_approval: bool,
    pub max_elections_active: Option<u32>,
    pub enable_anonymous_voting: bool,
    pub enable_voter_receipts: bool,
    pub enable_result_transparency: bool,
    pub timezone: String,
    pub locale: String,
}

impl Default for OrganizationSettings {
    fn default() -> Self {
        Self {
            allow_public_registration: false,
            require_email_verification: true,
            require_admin_approval: true,
            max_elections_active: Some(5),
            enable_anonymous_voting: true,
            enable_voter_receipts: true,
            enable_result_transparency: true,
            timezone: "UTC".to_string(),
            locale: "en-US".to_string(),
        }
    }
}

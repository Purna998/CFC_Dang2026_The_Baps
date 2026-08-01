//! Data Transfer Objects for Organization Service

use eemp_domain::TenantId;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{Organization, OrganizationSettings, OrganizationType};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrganizationRequest {
    #[validate(length(min = 2, max = 200))]
    pub name: String,
    pub organization_type: OrganizationType,
    #[validate(length(max = 100))]
    pub domain: Option<String>,
    #[validate(length(max = 50))]
    pub subdomain: Option<String>,
    #[validate(url)]
    pub logo_url: Option<String>,
    #[validate(url)]
    pub website: Option<String>,
    #[validate(email)]
    pub contact_email: String,
    #[validate(length(max = 20))]
    pub contact_phone: Option<String>,
    #[validate(length(max = 200))]
    pub address: Option<String>,
    #[validate(length(max = 100))]
    pub city: Option<String>,
    #[validate(length(max = 100))]
    pub state: Option<String>,
    #[validate(length(min = 2, max = 100))]
    pub country: String,
    #[validate(length(max = 20))]
    pub postal_code: Option<String>,
    pub settings: Option<OrganizationSettings>,
}

#[derive(Debug, Serialize)]
pub struct CreateOrganizationResponse {
    pub tenant_id: TenantId,
    pub name: String,
    pub organization_type: OrganizationType,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrganizationRequest {
    #[validate(length(min = 2, max = 200))]
    pub name: Option<String>,
    #[validate(url)]
    pub logo_url: Option<String>,
    #[validate(url)]
    pub website: Option<String>,
    #[validate(email)]
    pub contact_email: Option<String>,
    #[validate(length(max = 20))]
    pub contact_phone: Option<String>,
    #[validate(length(max = 200))]
    pub address: Option<String>,
    #[validate(length(max = 100))]
    pub city: Option<String>,
    #[validate(length(max = 100))]
    pub state: Option<String>,
    #[validate(length(max = 100))]
    pub country: Option<String>,
    #[validate(length(max = 20))]
    pub postal_code: Option<String>,
    pub settings: Option<OrganizationSettings>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationResponse {
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
    pub created_at: String,
    pub updated_at: String,
}

impl From<Organization> for OrganizationResponse {
    fn from(org: Organization) -> Self {
        Self {
            tenant_id: org.tenant_id,
            name: org.name,
            organization_type: org.organization_type,
            domain: org.domain,
            subdomain: org.subdomain,
            logo_url: org.logo_url,
            website: org.website,
            contact_email: org.contact_email,
            contact_phone: org.contact_phone,
            address: org.address,
            city: org.city,
            state: org.state,
            country: org.country,
            postal_code: org.postal_code,
            settings: org.settings,
            is_active: org.is_active,
            created_at: org.created_at.to_rfc3339(),
            updated_at: org.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OrganizationListResponse {
    pub organizations: Vec<OrganizationResponse>,
    pub total: i64,
}

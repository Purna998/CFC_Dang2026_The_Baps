//! Organization service - business logic layer

use eemp_database::Database;
use eemp_domain::TenantId;
use eemp_error::{AppError, Result};

use crate::{
    dto::*,
    models::{Organization, OrganizationSettings},
    repository::OrganizationRepository,
};

pub struct OrganizationService {
    repository: OrganizationRepository,
}

impl OrganizationService {
    /// Create a new organization service
    pub fn new(db: Database) -> Self {
        Self {
            repository: OrganizationRepository::new(db),
        }
    }

    /// Create a new organization
    pub async fn create_organization(
        &self,
        req: CreateOrganizationRequest,
    ) -> Result<CreateOrganizationResponse> {
        // Validate domain uniqueness if provided
        if let Some(ref domain) = req.domain {
            if let Some(_) = self.repository.get_by_domain(domain).await? {
                return Err(AppError::Conflict(format!(
                    "Domain '{}' is already registered",
                    domain
                )));
            }
        }

        // Validate subdomain uniqueness if provided
        if let Some(ref subdomain) = req.subdomain {
            if let Some(_) = self.repository.get_by_subdomain(subdomain).await? {
                return Err(AppError::Conflict(format!(
                    "Subdomain '{}' is already taken",
                    subdomain
                )));
            }
        }

        let settings = req.settings.unwrap_or_default();

        let organization = self
            .repository
            .create(
                &req.name,
                req.organization_type.clone(),
                req.domain.as_deref(),
                req.subdomain.as_deref(),
                req.logo_url.as_deref(),
                req.website.as_deref(),
                &req.contact_email,
                req.contact_phone.as_deref(),
                req.address.as_deref(),
                req.city.as_deref(),
                req.state.as_deref(),
                &req.country,
                req.postal_code.as_deref(),
                &settings,
            )
            .await?;

        tracing::info!(
            tenant_id = %organization.tenant_id,
            name = %organization.name,
            "Organization created"
        );

        Ok(CreateOrganizationResponse {
            tenant_id: organization.tenant_id,
            name: organization.name,
            organization_type: organization.organization_type,
        })
    }

    /// Get organization by ID
    pub async fn get_organization(&self, tenant_id: TenantId) -> Result<OrganizationResponse> {
        let organization = self
            .repository
            .get_by_id(tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        Ok(OrganizationResponse::from(organization))
    }

    /// Get organization by domain
    pub async fn get_organization_by_domain(&self, domain: &str) -> Result<OrganizationResponse> {
        let organization = self
            .repository
            .get_by_domain(domain)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        Ok(OrganizationResponse::from(organization))
    }

    /// Get organization by subdomain
    pub async fn get_organization_by_subdomain(
        &self,
        subdomain: &str,
    ) -> Result<OrganizationResponse> {
        let organization = self
            .repository
            .get_by_subdomain(subdomain)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        Ok(OrganizationResponse::from(organization))
    }

    /// Update organization
    pub async fn update_organization(
        &self,
        tenant_id: TenantId,
        req: UpdateOrganizationRequest,
    ) -> Result<OrganizationResponse> {
        let organization = self
            .repository
            .update(
                tenant_id,
                req.name.as_deref(),
                req.logo_url.as_deref(),
                req.website.as_deref(),
                req.contact_email.as_deref(),
                req.contact_phone.as_deref(),
                req.address.as_deref(),
                req.city.as_deref(),
                req.state.as_deref(),
                req.country.as_deref(),
                req.postal_code.as_deref(),
                req.settings.as_ref(),
                req.is_active,
            )
            .await?;

        tracing::info!(
            tenant_id = %tenant_id,
            "Organization updated"
        );

        Ok(OrganizationResponse::from(organization))
    }

    /// List organizations (platform admin only)
    pub async fn list_organizations(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<OrganizationListResponse> {
        let (organizations, total) = self.repository.list(limit, offset).await?;

        Ok(OrganizationListResponse {
            organizations: organizations
                .into_iter()
                .map(OrganizationResponse::from)
                .collect(),
            total,
        })
    }

    /// Delete organization (soft delete)
    pub async fn delete_organization(&self, tenant_id: TenantId) -> Result<()> {
        self.repository.delete(tenant_id).await?;

        tracing::info!(
            tenant_id = %tenant_id,
            "Organization deleted (soft delete)"
        );

        Ok(())
    }

    /// Resolve tenant ID from domain
    pub async fn resolve_tenant_from_domain(&self, domain: &str) -> Result<TenantId> {
        let organization = self
            .repository
            .get_by_domain(domain)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found for domain".to_string()))?;

        Ok(organization.tenant_id)
    }

    /// Resolve tenant ID from subdomain
    pub async fn resolve_tenant_from_subdomain(&self, subdomain: &str) -> Result<TenantId> {
        let organization = self
            .repository
            .get_by_subdomain(subdomain)
            .await?
            .ok_or_else(|| {
                AppError::NotFound("Organization not found for subdomain".to_string())
            })?;

        Ok(organization.tenant_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests require database
    #[tokio::test]
    #[ignore]
    async fn test_create_organization() {
        // Placeholder for integration tests
    }
}

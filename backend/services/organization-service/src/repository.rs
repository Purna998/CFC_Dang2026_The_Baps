//! Organization repository - data access layer

use chrono::Utc;
use eemp_database::Database;
use eemp_domain::TenantId;
use eemp_error::{AppError, Result};
use sqlx::PgPool;

use crate::models::{Organization, OrganizationSettings, OrganizationType};

pub struct OrganizationRepository {
    db: Database,
}

impl OrganizationRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Create a new organization
    pub async fn create(
        &self,
        name: &str,
        organization_type: OrganizationType,
        domain: Option<&str>,
        subdomain: Option<&str>,
        logo_url: Option<&str>,
        website: Option<&str>,
        contact_email: &str,
        contact_phone: Option<&str>,
        address: Option<&str>,
        city: Option<&str>,
        state: Option<&str>,
        country: &str,
        postal_code: Option<&str>,
        settings: &OrganizationSettings,
    ) -> Result<Organization> {
        let tenant_id = TenantId::new();
        let now = Utc::now();
        let settings_json = serde_json::to_value(settings)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize settings: {}", e)))?;

        let row = sqlx::query!(
            r#"
            INSERT INTO organizations (
                tenant_id, name, organization_type, domain, subdomain,
                logo_url, website, contact_email, contact_phone,
                address, city, state, country, postal_code,
                settings, is_active, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            RETURNING
                tenant_id, name, organization_type, domain, subdomain,
                logo_url, website, contact_email, contact_phone,
                address, city, state, country, postal_code,
                settings, is_active, created_at, updated_at
            "#,
            tenant_id.as_uuid(),
            name,
            organization_type.to_string(),
            domain,
            subdomain,
            logo_url,
            website,
            contact_email,
            contact_phone,
            address,
            city,
            state,
            country,
            postal_code,
            settings_json,
            true, // is_active
            now,
            now,
        )
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create organization: {}", e)))?;

        let settings: OrganizationSettings = serde_json::from_value(row.settings)
            .map_err(|e| AppError::InternalError(format!("Failed to parse settings: {}", e)))?;

        Ok(Organization {
            tenant_id: TenantId::from_uuid(row.tenant_id),
            name: row.name,
            organization_type: row.organization_type.parse()
                .map_err(|e: String| AppError::InternalError(e))?,
            domain: row.domain,
            subdomain: row.subdomain,
            logo_url: row.logo_url,
            website: row.website,
            contact_email: row.contact_email,
            contact_phone: row.contact_phone,
            address: row.address,
            city: row.city,
            state: row.state,
            country: row.country,
            postal_code: row.postal_code,
            settings,
            is_active: row.is_active,
            created_at: row.created_at.and_utc(),
            updated_at: row.updated_at.and_utc(),
        })
    }

    /// Get organization by tenant ID
    pub async fn get_by_id(&self, tenant_id: TenantId) -> Result<Option<Organization>> {
        let row = sqlx::query!(
            r#"
            SELECT
                tenant_id, name, organization_type, domain, subdomain,
                logo_url, website, contact_email, contact_phone,
                address, city, state, country, postal_code,
                settings, is_active, created_at, updated_at
            FROM organizations
            WHERE tenant_id = $1
            "#,
            tenant_id.as_uuid()
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get organization: {}", e)))?;

        match row {
            Some(row) => {
                let settings: OrganizationSettings = serde_json::from_value(row.settings)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse settings: {}", e)))?;

                Ok(Some(Organization {
                    tenant_id: TenantId::from_uuid(row.tenant_id),
                    name: row.name,
                    organization_type: row.organization_type.parse()
                        .map_err(|e: String| AppError::InternalError(e))?,
                    domain: row.domain,
                    subdomain: row.subdomain,
                    logo_url: row.logo_url,
                    website: row.website,
                    contact_email: row.contact_email,
                    contact_phone: row.contact_phone,
                    address: row.address,
                    city: row.city,
                    state: row.state,
                    country: row.country,
                    postal_code: row.postal_code,
                    settings,
                    is_active: row.is_active,
                    created_at: row.created_at.and_utc(),
                    updated_at: row.updated_at.and_utc(),
                }))
            }
            None => Ok(None),
        }
    }

    /// Get organization by domain
    pub async fn get_by_domain(&self, domain: &str) -> Result<Option<Organization>> {
        let row = sqlx::query!(
            r#"
            SELECT
                tenant_id, name, organization_type, domain, subdomain,
                logo_url, website, contact_email, contact_phone,
                address, city, state, country, postal_code,
                settings, is_active, created_at, updated_at
            FROM organizations
            WHERE domain = $1 AND is_active = true
            "#,
            domain
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get organization by domain: {}", e)))?;

        match row {
            Some(row) => {
                let settings: OrganizationSettings = serde_json::from_value(row.settings)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse settings: {}", e)))?;

                Ok(Some(Organization {
                    tenant_id: TenantId::from_uuid(row.tenant_id),
                    name: row.name,
                    organization_type: row.organization_type.parse()
                        .map_err(|e: String| AppError::InternalError(e))?,
                    domain: row.domain,
                    subdomain: row.subdomain,
                    logo_url: row.logo_url,
                    website: row.website,
                    contact_email: row.contact_email,
                    contact_phone: row.contact_phone,
                    address: row.address,
                    city: row.city,
                    state: row.state,
                    country: row.country,
                    postal_code: row.postal_code,
                    settings,
                    is_active: row.is_active,
                    created_at: row.created_at.and_utc(),
                    updated_at: row.updated_at.and_utc(),
                }))
            }
            None => Ok(None),
        }
    }

    /// Get organization by subdomain
    pub async fn get_by_subdomain(&self, subdomain: &str) -> Result<Option<Organization>> {
        let row = sqlx::query!(
            r#"
            SELECT
                tenant_id, name, organization_type, domain, subdomain,
                logo_url, website, contact_email, contact_phone,
                address, city, state, country, postal_code,
                settings, is_active, created_at, updated_at
            FROM organizations
            WHERE subdomain = $1 AND is_active = true
            "#,
            subdomain
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get organization by subdomain: {}", e)))?;

        match row {
            Some(row) => {
                let settings: OrganizationSettings = serde_json::from_value(row.settings)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse settings: {}", e)))?;

                Ok(Some(Organization {
                    tenant_id: TenantId::from_uuid(row.tenant_id),
                    name: row.name,
                    organization_type: row.organization_type.parse()
                        .map_err(|e: String| AppError::InternalError(e))?,
                    domain: row.domain,
                    subdomain: row.subdomain,
                    logo_url: row.logo_url,
                    website: row.website,
                    contact_email: row.contact_email,
                    contact_phone: row.contact_phone,
                    address: row.address,
                    city: row.city,
                    state: row.state,
                    country: row.country,
                    postal_code: row.postal_code,
                    settings,
                    is_active: row.is_active,
                    created_at: row.created_at.and_utc(),
                    updated_at: row.updated_at.and_utc(),
                }))
            }
            None => Ok(None),
        }
    }

    /// Update organization
    pub async fn update(
        &self,
        tenant_id: TenantId,
        name: Option<&str>,
        logo_url: Option<&str>,
        website: Option<&str>,
        contact_email: Option<&str>,
        contact_phone: Option<&str>,
        address: Option<&str>,
        city: Option<&str>,
        state: Option<&str>,
        country: Option<&str>,
        postal_code: Option<&str>,
        settings: Option<&OrganizationSettings>,
        is_active: Option<bool>,
    ) -> Result<Organization> {
        let now = Utc::now();

        // Build dynamic update query (simplified for readability)
        // In production, use a query builder or handle optional fields properly

        let current = self.get_by_id(tenant_id).await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        let updated_name = name.unwrap_or(&current.name);
        let updated_logo_url = logo_url.or(current.logo_url.as_deref());
        let updated_website = website.or(current.website.as_deref());
        let updated_contact_email = contact_email.unwrap_or(&current.contact_email);
        let updated_contact_phone = contact_phone.or(current.contact_phone.as_deref());
        let updated_address = address.or(current.address.as_deref());
        let updated_city = city.or(current.city.as_deref());
        let updated_state = state.or(current.state.as_deref());
        let updated_country = country.unwrap_or(&current.country);
        let updated_postal_code = postal_code.or(current.postal_code.as_deref());
        let updated_settings = settings.unwrap_or(&current.settings);
        let updated_is_active = is_active.unwrap_or(current.is_active);

        let settings_json = serde_json::to_value(updated_settings)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize settings: {}", e)))?;

        sqlx::query!(
            r#"
            UPDATE organizations
            SET
                name = $2,
                logo_url = $3,
                website = $4,
                contact_email = $5,
                contact_phone = $6,
                address = $7,
                city = $8,
                state = $9,
                country = $10,
                postal_code = $11,
                settings = $12,
                is_active = $13,
                updated_at = $14
            WHERE tenant_id = $1
            "#,
            tenant_id.as_uuid(),
            updated_name,
            updated_logo_url,
            updated_website,
            updated_contact_email,
            updated_contact_phone,
            updated_address,
            updated_city,
            updated_state,
            updated_country,
            updated_postal_code,
            settings_json,
            updated_is_active,
            now,
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update organization: {}", e)))?;

        // Fetch updated organization
        self.get_by_id(tenant_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found after update".to_string()))
    }

    /// List all organizations (platform admin only)
    pub async fn list(&self, limit: i64, offset: i64) -> Result<(Vec<Organization>, i64)> {
        let rows = sqlx::query!(
            r#"
            SELECT
                tenant_id, name, organization_type, domain, subdomain,
                logo_url, website, contact_email, contact_phone,
                address, city, state, country, postal_code,
                settings, is_active, created_at, updated_at
            FROM organizations
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to list organizations: {}", e)))?;

        let total = sqlx::query_scalar!("SELECT COUNT(*) FROM organizations")
            .fetch_one(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to count organizations: {}", e)))?
            .unwrap_or(0);

        let organizations = rows
            .into_iter()
            .map(|row| {
                let settings: OrganizationSettings = serde_json::from_value(row.settings)
                    .map_err(|e| AppError::InternalError(format!("Failed to parse settings: {}", e)))?;

                Ok(Organization {
                    tenant_id: TenantId::from_uuid(row.tenant_id),
                    name: row.name,
                    organization_type: row.organization_type.parse()
                        .map_err(|e: String| AppError::InternalError(e))?,
                    domain: row.domain,
                    subdomain: row.subdomain,
                    logo_url: row.logo_url,
                    website: row.website,
                    contact_email: row.contact_email,
                    contact_phone: row.contact_phone,
                    address: row.address,
                    city: row.city,
                    state: row.state,
                    country: row.country,
                    postal_code: row.postal_code,
                    settings,
                    is_active: row.is_active,
                    created_at: row.created_at.and_utc(),
                    updated_at: row.updated_at.and_utc(),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok((organizations, total))
    }

    /// Delete organization (soft delete - mark as inactive)
    pub async fn delete(&self, tenant_id: TenantId) -> Result<()> {
        let result = sqlx::query!(
            "UPDATE organizations SET is_active = false, updated_at = NOW() WHERE tenant_id = $1",
            tenant_id.as_uuid()
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete organization: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Organization not found".to_string()));
        }

        Ok(())
    }
}

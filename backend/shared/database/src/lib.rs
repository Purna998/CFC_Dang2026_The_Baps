//! Database utilities for EEMP
//!
//! Provides:
//! - Database connection pool management
//! - Row-Level Security (RLS) helper for multi-tenancy
//! - Common database operations

use eemp_config::DatabaseConfig;
use eemp_domain::TenantId;
use eemp_error::{AppError, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Database pool wrapper with multi-tenancy support
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(&config.url)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to connect to database: {}", e)))?;

        tracing::info!(
            "Database pool created: max_connections={}, min_connections={}",
            config.max_connections,
            config.min_connections
        );

        Ok(Self { pool })
    }

    /// Get the underlying SQLx pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Set tenant context for Row-Level Security
    ///
    /// This MUST be called at the start of every request to enforce multi-tenancy.
    ///
    /// # Example
    /// ```rust,ignore
    /// db.set_tenant_context(tenant_id).await?;
    /// // All subsequent queries will only see data for this tenant
    /// ```
    pub async fn set_tenant_context(&self, tenant_id: TenantId) -> Result<()> {
        sqlx::query("SET LOCAL app.current_tenant_id = $1")
            .bind(tenant_id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to set tenant context: {}", e))
            })?;

        Ok(())
    }

    /// Clear tenant context (use with caution - for platform admin only)
    pub async fn clear_tenant_context(&self) -> Result<()> {
        sqlx::query("RESET app.current_tenant_id")
            .execute(&self.pool)
            .await
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to clear tenant context: {}", e))
            })?;

        Ok(())
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("../migrations")
            .run(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Migration failed: {}", e)))?;

        tracing::info!("Database migrations completed successfully");

        Ok(())
    }

    /// Health check - verify database connection
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Health check failed: {}", e)))?;

        Ok(())
    }

    /// Close the database pool gracefully
    pub async fn close(&self) {
        self.pool.close().await;
        tracing::info!("Database pool closed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires database connection
    async fn test_database_connection() {
        let config = DatabaseConfig {
            url: "postgres://localhost/eemp_test".to_string(),
            max_connections: 5,
            min_connections: 1,
        };

        let db = Database::new(&config).await.unwrap();
        db.health_check().await.unwrap();
        db.close().await;
    }
}

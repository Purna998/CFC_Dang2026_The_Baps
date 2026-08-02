//! Session management with Redis and PostgreSQL
//!
//! Hybrid approach:
//! - Active sessions in Redis (fast access)
//! - Persistent sessions in PostgreSQL (audit trail)

use chrono::{DateTime, Duration, Utc};
use eemp_domain::{TenantId, UserId};
use eemp_error::{AppError, Result};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub refresh_token_hash: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

pub struct SessionManager {
    redis: redis::Client,
    db_pool: PgPool,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(redis_url: &str, db_pool: PgPool) -> Result<Self> {
        let redis = redis::Client::open(redis_url)
            .map_err(|e| AppError::InternalError(format!("Failed to connect to Redis: {}", e)))?;

        Ok(Self { redis, db_pool })
    }

    /// Create a new session
    pub async fn create_session(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
        refresh_token_hash: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
        ttl_seconds: i64,
    ) -> Result<Session> {
        let session_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + Duration::seconds(ttl_seconds);

        let session = Session {
            session_id: session_id.clone(),
            user_id,
            tenant_id,
            refresh_token_hash: refresh_token_hash.clone(),
            created_at: now,
            expires_at,
            last_activity: now,
            ip_address: ip_address.clone(),
            user_agent: user_agent.clone(),
        };

        // Store in Redis for fast access
        let mut conn = self
            .redis
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::InternalError(format!("Redis connection failed: {}", e)))?;

        let session_json = serde_json::to_string(&session)
            .map_err(|e| AppError::InternalError(format!("Session serialization failed: {}", e)))?;

        conn.set_ex::<_, _, ()>(
            format!("session:{}", session_id),
            session_json,
            ttl_seconds as u64,
        )
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to store session in Redis: {}", e)))?;

        // Store in PostgreSQL for persistence and audit trail
        let session_uuid = Uuid::parse_str(&session_id)
            .map_err(|e| AppError::InternalError(format!("Invalid session UUID: {}", e)))?;

        sqlx::query!(
            r#"
            INSERT INTO sessions (
                session_id, user_id, tenant_id, refresh_token_hash,
                created_at, expires_at, last_activity, ip_address, user_agent
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            session_uuid,
            user_id.as_uuid(),
            tenant_id.as_uuid(),
            refresh_token_hash,
            now,
            expires_at,
            now,
            ip_address,
            user_agent,
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to store session in database: {}", e)))?;

        tracing::info!(
            session_id = %session_id,
            user_id = %user_id,
            "Session created"
        );

        Ok(session)
    }

    /// Get a session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<Option<Session>> {
        // Try Redis first (fast path)
        let mut conn = self
            .redis
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::InternalError(format!("Redis connection failed: {}", e)))?;

        let session_json: Option<String> = conn
            .get(format!("session:{}", session_id))
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to get session from Redis: {}", e)))?;

        if let Some(json) = session_json {
            let session: Session = serde_json::from_str(&json)
                .map_err(|e| AppError::InternalError(format!("Session deserialization failed: {}", e)))?;
            return Ok(Some(session));
        }

        // Fallback to PostgreSQL (slow path)
        let session_uuid = Uuid::parse_str(session_id)
            .map_err(|e| AppError::InternalError(format!("Invalid session UUID: {}", e)))?;

        let row = sqlx::query!(
            r#"
            SELECT session_id, user_id, tenant_id, refresh_token_hash,
                   created_at, expires_at, last_activity, ip_address, user_agent
            FROM sessions
            WHERE session_id = $1 AND expires_at > NOW()
            "#,
            session_uuid
        )
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get session from database: {}", e)))?;

        Ok(row.map(|r| Session {
            session_id: r.session_id.to_string(),
            user_id: UserId::from_uuid(r.user_id),
            tenant_id: TenantId::from_uuid(r.tenant_id),
            refresh_token_hash: r.refresh_token_hash,
            created_at: r.created_at,
            expires_at: r.expires_at,
            last_activity: r.last_activity,
            ip_address: r.ip_address,
            user_agent: r.user_agent,
        }))
    }

    /// Update session last activity
    pub async fn update_last_activity(&self, session_id: &str) -> Result<()> {
        let now = Utc::now();
        let session_uuid = Uuid::parse_str(session_id)
            .map_err(|e| AppError::InternalError(format!("Invalid session UUID: {}", e)))?;

        // Update in PostgreSQL
        sqlx::query!(
            "UPDATE sessions SET last_activity = $1 WHERE session_id = $2",
            now,
            session_uuid
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update session: {}", e)))?;

        Ok(())
    }

    /// Delete a session (logout)
    pub async fn delete_session(&self, session_id: &str) -> Result<()> {
        let session_uuid = Uuid::parse_str(session_id)
            .map_err(|e| AppError::InternalError(format!("Invalid session UUID: {}", e)))?;

        // Delete from Redis
        let mut conn = self
            .redis
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::InternalError(format!("Redis connection failed: {}", e)))?;

        conn.del::<_, ()>(format!("session:{}", session_id))
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to delete session from Redis: {}", e)))?;

        // Mark as revoked in PostgreSQL (keep for audit trail)
        sqlx::query!(
            "UPDATE sessions SET expires_at = NOW() WHERE session_id = $1",
            session_uuid
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to revoke session: {}", e)))?;

        tracing::info!(session_id = %session_id, "Session deleted");

        Ok(())
    }

    /// Delete all sessions for a user (logout from all devices)
    pub async fn delete_user_sessions(&self, user_id: UserId) -> Result<()> {
        // Get all session IDs for the user
        let rows = sqlx::query!(
            "SELECT session_id FROM sessions WHERE user_id = $1 AND expires_at > NOW()",
            user_id.as_uuid()
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get user sessions: {}", e)))?;

        // Delete from Redis
        let mut conn = self
            .redis
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::InternalError(format!("Redis connection failed: {}", e)))?;

        for row in rows {
            conn.del::<_, ()>(format!("session:{}", row.session_id))
                .await
                .map_err(|e| AppError::InternalError(format!("Failed to delete session from Redis: {}", e)))?;
        }

        // Mark all as revoked in PostgreSQL
        sqlx::query!(
            "UPDATE sessions SET expires_at = NOW() WHERE user_id = $1 AND expires_at > NOW()",
            user_id.as_uuid()
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to revoke user sessions: {}", e)))?;

        tracing::info!(user_id = %user_id, "All user sessions deleted");

        Ok(())
    }

    /// Clean up expired sessions from PostgreSQL
    pub async fn cleanup_expired_sessions(&self) -> Result<u64> {
        let result = sqlx::query!(
            "DELETE FROM sessions WHERE expires_at < NOW() - INTERVAL '30 days'"
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to cleanup expired sessions: {}", e)))?;

        let deleted = result.rows_affected();
        if deleted > 0 {
            tracing::info!(deleted = deleted, "Expired sessions cleaned up");
        }

        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests require Redis and PostgreSQL
    // Run with: cargo test --features integration-tests

    #[tokio::test]
    #[ignore]
    async fn test_create_and_get_session() {
        // Setup test database and Redis
        // This is a placeholder - real tests would use testcontainers
    }
}

//! Election repository - data access layer

use chrono::Utc;
use eemp_database::Database;
use eemp_domain::{ElectionId, ElectionStatus, ElectionType, TenantId, UserId};
use eemp_error::{AppError, Result};
use uuid::Uuid;

use crate::models::{Candidate, Election, EligibilityRule, EligibilityRuleType, Position};

pub struct ElectionRepository {
    db: Database,
}

impl ElectionRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Create a new election
    pub async fn create_election(
        &self,
        tenant_id: TenantId,
        title: &str,
        description: Option<&str>,
        election_type: ElectionType,
        voting_start_time: chrono::DateTime<Utc>,
        voting_end_time: chrono::DateTime<Utc>,
        result_publish_time: Option<chrono::DateTime<Utc>>,
        allow_write_in_candidates: bool,
        allow_abstain: bool,
        require_identity_verification: bool,
        enable_blockchain_verification: bool,
        max_votes_per_voter: Option<i32>,
        created_by: UserId,
    ) -> Result<Election> {
        let election_id = ElectionId::new();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO elections (
                election_id, tenant_id, title, description, election_type,
                status, voting_start_time, voting_end_time, result_publish_time,
                allow_write_in_candidates, allow_abstain, require_identity_verification,
                enable_blockchain_verification, max_votes_per_voter, created_by,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            "#,
            election_id.as_uuid(),
            tenant_id.as_uuid(),
            title,
            description,
            election_type.to_string(),
            ElectionStatus::Draft.to_string(),
            voting_start_time,
            voting_end_time,
            result_publish_time,
            allow_write_in_candidates,
            allow_abstain,
            require_identity_verification,
            enable_blockchain_verification,
            max_votes_per_voter,
            created_by.as_uuid(),
            now,
            now,
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create election: {}", e)))?;

        self.get_by_id(election_id)
            .await?
            .ok_or_else(|| AppError::InternalError("Election not found after creation".to_string()))
    }

    /// Get election by ID
    pub async fn get_by_id(&self, election_id: ElectionId) -> Result<Option<Election>> {
        let row = sqlx::query!(
            r#"
            SELECT
                election_id, tenant_id, title, description, election_type, status,
                voting_start_time, voting_end_time, result_publish_time,
                allow_write_in_candidates, allow_abstain, require_identity_verification,
                enable_blockchain_verification, max_votes_per_voter, created_by,
                created_at, updated_at, published_at, archived_at
            FROM elections
            WHERE election_id = $1
            "#,
            election_id.as_uuid()
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get election: {}", e)))?;

        match row {
            Some(row) => Ok(Some(Election {
                election_id: ElectionId::from_uuid(row.election_id),
                tenant_id: TenantId::from_uuid(row.tenant_id),
                title: row.title,
                description: row.description,
                election_type: row
                    .election_type
                    .parse()
                    .map_err(|e: String| AppError::InternalError(e))?,
                status: row
                    .status
                    .parse()
                    .map_err(|e: String| AppError::InternalError(e))?,
                voting_start_time: row.voting_start_time.and_utc(),
                voting_end_time: row.voting_end_time.and_utc(),
                result_publish_time: row.result_publish_time.map(|dt| dt.and_utc()),
                allow_write_in_candidates: row.allow_write_in_candidates,
                allow_abstain: row.allow_abstain,
                require_identity_verification: row.require_identity_verification,
                enable_blockchain_verification: row.enable_blockchain_verification,
                max_votes_per_voter: row.max_votes_per_voter,
                created_by: UserId::from_uuid(row.created_by),
                created_at: row.created_at.and_utc(),
                updated_at: row.updated_at.and_utc(),
                published_at: row.published_at.map(|dt| dt.and_utc()),
                archived_at: row.archived_at.map(|dt| dt.and_utc()),
            })),
            None => Ok(None),
        }
    }

    /// Update election status
    pub async fn update_status(
        &self,
        election_id: ElectionId,
        new_status: ElectionStatus,
    ) -> Result<()> {
        let now = Utc::now();

        sqlx::query!(
            "UPDATE elections SET status = $1, updated_at = $2 WHERE election_id = $3",
            new_status.to_string(),
            now,
            election_id.as_uuid()
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to update election status: {}", e)))?;

        Ok(())
    }

    /// List elections for tenant
    pub async fn list_by_tenant(
        &self,
        tenant_id: TenantId,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<Election>, i64)> {
        let rows = sqlx::query!(
            r#"
            SELECT
                election_id, tenant_id, title, description, election_type, status,
                voting_start_time, voting_end_time, result_publish_time,
                allow_write_in_candidates, allow_abstain, require_identity_verification,
                enable_blockchain_verification, max_votes_per_voter, created_by,
                created_at, updated_at, published_at, archived_at
            FROM elections
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id.as_uuid(),
            limit,
            offset
        )
        .fetch_all(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to list elections: {}", e)))?;

        let total = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM elections WHERE tenant_id = $1",
            tenant_id.as_uuid()
        )
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to count elections: {}", e)))?
        .unwrap_or(0);

        let elections = rows
            .into_iter()
            .map(|row| {
                Ok(Election {
                    election_id: ElectionId::from_uuid(row.election_id),
                    tenant_id: TenantId::from_uuid(row.tenant_id),
                    title: row.title,
                    description: row.description,
                    election_type: row
                        .election_type
                        .parse()
                        .map_err(|e: String| AppError::InternalError(e))?,
                    status: row
                        .status
                        .parse()
                        .map_err(|e: String| AppError::InternalError(e))?,
                    voting_start_time: row.voting_start_time.and_utc(),
                    voting_end_time: row.voting_end_time.and_utc(),
                    result_publish_time: row.result_publish_time.map(|dt| dt.and_utc()),
                    allow_write_in_candidates: row.allow_write_in_candidates,
                    allow_abstain: row.allow_abstain,
                    require_identity_verification: row.require_identity_verification,
                    enable_blockchain_verification: row.enable_blockchain_verification,
                    max_votes_per_voter: row.max_votes_per_voter,
                    created_by: UserId::from_uuid(row.created_by),
                    created_at: row.created_at.and_utc(),
                    updated_at: row.updated_at.and_utc(),
                    published_at: row.published_at.map(|dt| dt.and_utc()),
                    archived_at: row.archived_at.map(|dt| dt.and_utc()),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok((elections, total))
    }

    /// Create a position
    pub async fn create_position(
        &self,
        tenant_id: TenantId,
        election_id: ElectionId,
        title: &str,
        description: Option<&str>,
        display_order: i32,
        seats_available: i32,
        min_votes_required: Option<i32>,
        max_votes_per_voter: Option<i32>,
    ) -> Result<Position> {
        let position_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO positions (
                position_id, tenant_id, election_id, title, description,
                display_order, seats_available, min_votes_required,
                max_votes_per_voter, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            position_id,
            tenant_id.as_uuid(),
            election_id.as_uuid(),
            title,
            description,
            display_order,
            seats_available,
            min_votes_required,
            max_votes_per_voter,
            now,
            now,
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create position: {}", e)))?;

        Ok(Position {
            position_id,
            tenant_id,
            election_id,
            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            display_order,
            seats_available,
            min_votes_required,
            max_votes_per_voter,
            created_at: now,
            updated_at: now,
        })
    }

    /// List positions for election
    pub async fn list_positions(&self, election_id: ElectionId) -> Result<Vec<Position>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                position_id, tenant_id, election_id, title, description,
                display_order, seats_available, min_votes_required,
                max_votes_per_voter, created_at, updated_at
            FROM positions
            WHERE election_id = $1
            ORDER BY display_order ASC
            "#,
            election_id.as_uuid()
        )
        .fetch_all(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to list positions: {}", e)))?;

        Ok(rows
            .into_iter()
            .map(|row| Position {
                position_id: row.position_id,
                tenant_id: TenantId::from_uuid(row.tenant_id),
                election_id: ElectionId::from_uuid(row.election_id),
                title: row.title,
                description: row.description,
                display_order: row.display_order,
                seats_available: row.seats_available,
                min_votes_required: row.min_votes_required,
                max_votes_per_voter: row.max_votes_per_voter,
                created_at: row.created_at.and_utc(),
                updated_at: row.updated_at.and_utc(),
            })
            .collect())
    }
}

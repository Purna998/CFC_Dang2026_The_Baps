//! Result repository - data access layer

use chrono::Utc;
use eemp_database::Database;
use eemp_domain::{ElectionId, TenantId};
use eemp_error::{AppError, Result};
use uuid::Uuid;

use crate::models::Result as ElectionResult;

pub struct ResultRepository {
    db: Database,
}

impl ResultRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Store election results
    pub async fn store_result(
        &self,
        tenant_id: TenantId,
        election_id: ElectionId,
        position_id: Uuid,
        candidate_id: Option<Uuid>,
        vote_count: i32,
        vote_percentage: f64,
        is_winner: bool,
        rank: i32,
    ) -> Result<ElectionResult> {
        let result_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO results (
                result_id, tenant_id, election_id, position_id, candidate_id,
                vote_count, vote_percentage, is_winner, rank, calculated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            result_id,
            tenant_id.as_uuid(),
            election_id.as_uuid(),
            position_id,
            candidate_id,
            vote_count,
            vote_percentage,
            is_winner,
            rank,
            now,
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to store result: {}", e)))?;

        Ok(ElectionResult {
            result_id,
            tenant_id,
            election_id,
            position_id,
            candidate_id,
            vote_count,
            vote_percentage: Some(vote_percentage),
            is_winner,
            rank: Some(rank),
            calculated_at: now,
            published_at: None,
        })
    }

    /// Get results for an election
    pub async fn get_election_results(
        &self,
        election_id: ElectionId,
    ) -> Result<Vec<ElectionResult>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                result_id, tenant_id, election_id, position_id, candidate_id,
                vote_count, vote_percentage, is_winner, rank,
                calculated_at, published_at
            FROM results
            WHERE election_id = $1
            ORDER BY position_id, rank
            "#,
            election_id.as_uuid()
        )
        .fetch_all(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get results: {}", e)))?;

        Ok(rows
            .into_iter()
            .map(|r| ElectionResult {
                result_id: r.result_id,
                tenant_id: TenantId::from_uuid(r.tenant_id),
                election_id: ElectionId::from_uuid(r.election_id),
                position_id: r.position_id,
                candidate_id: r.candidate_id,
                vote_count: r.vote_count,
                vote_percentage: r.vote_percentage,
                is_winner: r.is_winner,
                rank: r.rank,
                calculated_at: r.calculated_at.and_utc(),
                published_at: r.published_at.map(|dt| dt.and_utc()),
            })
            .collect())
    }

    /// Publish results (mark as published)
    pub async fn publish_results(&self, election_id: ElectionId) -> Result<()> {
        let now = Utc::now();

        sqlx::query!(
            "UPDATE results SET published_at = $1 WHERE election_id = $2 AND published_at IS NULL",
            now,
            election_id.as_uuid()
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to publish results: {}", e)))?;

        Ok(())
    }

    /// Delete existing results for an election (for recalculation)
    pub async fn delete_election_results(&self, election_id: ElectionId) -> Result<()> {
        sqlx::query!(
            "DELETE FROM results WHERE election_id = $1",
            election_id.as_uuid()
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to delete results: {}", e)))?;

        Ok(())
    }

    /// Get all encrypted ballots for an election
    pub async fn get_encrypted_ballots(
        &self,
        election_id: ElectionId,
    ) -> Result<Vec<(Uuid, String, Uuid)>> {
        let rows = sqlx::query!(
            "SELECT ballot_id, encrypted_ballot, encryption_key_id FROM ballots WHERE election_id = $1",
            election_id.as_uuid()
        )
        .fetch_all(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get ballots: {}", e)))?;

        Ok(rows
            .into_iter()
            .map(|r| (r.ballot_id, r.encrypted_ballot, r.encryption_key_id))
            .collect())
    }

    /// Get position details
    pub async fn get_positions(
        &self,
        election_id: ElectionId,
    ) -> Result<Vec<(Uuid, String, i32)>> {
        let rows = sqlx::query!(
            "SELECT position_id, title, seats_available FROM positions WHERE election_id = $1 ORDER BY display_order",
            election_id.as_uuid()
        )
        .fetch_all(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get positions: {}", e)))?;

        Ok(rows
            .into_iter()
            .map(|r| (r.position_id, r.title, r.seats_available))
            .collect())
    }

    /// Get candidate details
    pub async fn get_candidates(
        &self,
        election_id: ElectionId,
    ) -> Result<Vec<(Uuid, Uuid, String)>> {
        let rows = sqlx::query!(
            "SELECT candidate_id, position_id, full_name FROM candidates WHERE election_id = $1",
            election_id.as_uuid()
        )
        .fetch_all(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get candidates: {}", e)))?;

        Ok(rows
            .into_iter()
            .map(|r| (r.candidate_id, r.position_id, r.full_name))
            .collect())
    }
}

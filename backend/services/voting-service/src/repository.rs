//! Voting repository - data access layer

use chrono::Utc;
use eemp_database::Database;
use eemp_domain::{ElectionId, TenantId, UserId};
use eemp_error::{AppError, Result};
use uuid::Uuid;

use crate::models::{Ballot, VoteCommitment};

pub struct VotingRepository {
    db: Database,
}

impl VotingRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Store encrypted ballot
    pub async fn store_ballot(
        &self,
        tenant_id: TenantId,
        election_id: ElectionId,
        voter_id: UserId,
        encrypted_ballot: &str,
        encryption_key_id: Uuid,
        ballot_hash: &str,
        receipt_code: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<Ballot> {
        let ballot_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO ballots (
                ballot_id, tenant_id, election_id, voter_id,
                encrypted_ballot, encryption_key_id, ballot_hash,
                voter_receipt_code, ip_address, user_agent, cast_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            ballot_id,
            tenant_id.as_uuid(),
            election_id.as_uuid(),
            voter_id.as_uuid(),
            encrypted_ballot,
            encryption_key_id,
            ballot_hash,
            receipt_code,
            ip_address,
            user_agent,
            now,
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| {
            if e.to_string().contains("unique") {
                AppError::AlreadyVoted
            } else {
                AppError::DatabaseError(format!("Failed to store ballot: {}", e))
            }
        })?;

        Ok(Ballot {
            ballot_id,
            tenant_id,
            election_id,
            voter_id,
            encrypted_ballot: encrypted_ballot.to_string(),
            encryption_key_id,
            ballot_hash: ballot_hash.to_string(),
            voter_receipt_code: receipt_code.to_string(),
            ip_address: ip_address.map(|s| s.to_string()),
            user_agent: user_agent.map(|s| s.to_string()),
            cast_at: now,
            verified_at: None,
        })
    }

    /// Get ballot by receipt code
    pub async fn get_ballot_by_receipt(&self, receipt_code: &str) -> Result<Option<Ballot>> {
        let row = sqlx::query!(
            r#"
            SELECT
                ballot_id, tenant_id, election_id, voter_id,
                encrypted_ballot, encryption_key_id, ballot_hash,
                voter_receipt_code, ip_address, user_agent,
                cast_at, verified_at
            FROM ballots
            WHERE voter_receipt_code = $1
            "#,
            receipt_code
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get ballot: {}", e)))?;

        Ok(row.map(|r| Ballot {
            ballot_id: r.ballot_id,
            tenant_id: TenantId::from_uuid(r.tenant_id),
            election_id: ElectionId::from_uuid(r.election_id),
            voter_id: UserId::from_uuid(r.voter_id),
            encrypted_ballot: r.encrypted_ballot,
            encryption_key_id: r.encryption_key_id,
            ballot_hash: r.ballot_hash,
            voter_receipt_code: r.voter_receipt_code,
            ip_address: r.ip_address,
            user_agent: r.user_agent,
            cast_at: r.cast_at,
            verified_at: r.verified_at,
        }))
    }

    /// Check if user has voted in election
    pub async fn has_voted(&self, election_id: ElectionId, voter_id: UserId) -> Result<bool> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM ballots WHERE election_id = $1 AND voter_id = $2)",
            election_id.as_uuid(),
            voter_id.as_uuid()
        )
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to check voting status: {}", e)))?
        .unwrap_or(false);

        Ok(exists)
    }

    /// Get ballot for user in election
    pub async fn get_user_ballot(
        &self,
        election_id: ElectionId,
        voter_id: UserId,
    ) -> Result<Option<Ballot>> {
        let row = sqlx::query!(
            r#"
            SELECT
                ballot_id, tenant_id, election_id, voter_id,
                encrypted_ballot, encryption_key_id, ballot_hash,
                voter_receipt_code, ip_address, user_agent,
                cast_at, verified_at
            FROM ballots
            WHERE election_id = $1 AND voter_id = $2
            "#,
            election_id.as_uuid(),
            voter_id.as_uuid()
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get ballot: {}", e)))?;

        Ok(row.map(|r| Ballot {
            ballot_id: r.ballot_id,
            tenant_id: TenantId::from_uuid(r.tenant_id),
            election_id: ElectionId::from_uuid(r.election_id),
            voter_id: UserId::from_uuid(r.voter_id),
            encrypted_ballot: r.encrypted_ballot,
            encryption_key_id: r.encryption_key_id,
            ballot_hash: r.ballot_hash,
            voter_receipt_code: r.voter_receipt_code,
            ip_address: r.ip_address,
            user_agent: r.user_agent,
            cast_at: r.cast_at,
            verified_at: r.verified_at,
        }))
    }

    /// Store vote commitment for blockchain
    pub async fn store_commitment(
        &self,
        tenant_id: TenantId,
        election_id: ElectionId,
        ballot_id: Uuid,
        commitment_hash: &str,
        signature: &str,
    ) -> Result<VoteCommitment> {
        let commitment_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO vote_commitments (
                commitment_id, tenant_id, election_id, ballot_id,
                commitment_hash, blockchain_network, signature, submitted_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            commitment_id,
            tenant_id.as_uuid(),
            election_id.as_uuid(),
            ballot_id,
            commitment_hash,
            "solana",
            signature,
            now,
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to store commitment: {}", e)))?;

        Ok(VoteCommitment {
            commitment_id,
            tenant_id,
            election_id,
            ballot_id,
            commitment_hash: commitment_hash.to_string(),
            blockchain_transaction_id: None,
            blockchain_network: "solana".to_string(),
            blockchain_block_height: None,
            blockchain_timestamp: None,
            signature: signature.to_string(),
            submitted_at: now,
            confirmed_at: None,
        })
    }

    /// Count total ballots for election
    pub async fn count_ballots(&self, election_id: ElectionId) -> Result<i64> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM ballots WHERE election_id = $1",
            election_id.as_uuid()
        )
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to count ballots: {}", e)))?
        .unwrap_or(0);

        Ok(count)
    }
}

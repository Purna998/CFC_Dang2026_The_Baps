//! Voting service - business logic layer

use eemp_crypto_service::{
    encrypt_ballot, generate_encryption_key, generate_receipt_code, hash_ballot, sign_data,
    serialize_encrypted, KeyPair,
};
use eemp_database::Database;
use eemp_domain::{ElectionId, TenantId, UserId};
use eemp_error::{AppError, Result};

use crate::{
    dto::*,
    eligibility::{verify_voter_eligibility, verify_voting_time},
    models::BallotContent,
    repository::VotingRepository,
};

pub struct VotingService {
    repository: VotingRepository,
    db: Database,
}

impl VotingService {
    pub fn new(db: Database) -> Self {
        Self {
            repository: VotingRepository::new(db.clone()),
            db,
        }
    }

    /// Cast a ballot
    pub async fn cast_ballot(
        &self,
        tenant_id: TenantId,
        voter_id: UserId,
        req: CastBallotRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<CastBallotResponse> {
        // Verify voter eligibility
        verify_voter_eligibility(&self.db, req.election_id, voter_id).await?;

        // Verify voting time window
        verify_voting_time(&self.db, req.election_id).await?;

        // Validate votes
        self.validate_votes(&req).await?;

        // Create ballot content
        let ballot_content = BallotContent {
            election_id: req.election_id.to_string(),
            votes: req
                .votes
                .iter()
                .map(|v| crate::models::Vote {
                    position_id: v.position_id.to_string(),
                    candidate_ids: v.candidate_ids.iter().map(|c| c.to_string()).collect(),
                    is_abstain: v.is_abstain,
                })
                .collect(),
        };

        // Serialize ballot to JSON
        let ballot_json = serde_json::to_string(&ballot_content)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize ballot: {}", e)))?;

        // Generate encryption key (in production, use election-specific key from key management)
        let encryption_key = generate_encryption_key();

        // Encrypt ballot
        let encrypted = encrypt_ballot(ballot_json.as_bytes(), encryption_key.as_bytes())
            .map_err(|e| AppError::InternalError(format!("Encryption failed: {}", e)))?;

        let encrypted_str = serialize_encrypted(&encrypted);

        // Generate receipt code
        let receipt_code = generate_receipt_code();

        // Calculate ballot hash
        let ballot_hash = hash_ballot(
            &req.election_id.to_string(),
            &voter_id.to_string(),
            encrypted.ciphertext.as_slice(),
        )
        .map_err(|e| AppError::InternalError(format!("Hashing failed: {}", e)))?;

        // Store ballot
        let ballot = self
            .repository
            .store_ballot(
                tenant_id,
                req.election_id,
                voter_id,
                &encrypted_str,
                encryption_key.key_id,
                &ballot_hash,
                &receipt_code,
                ip_address.as_deref(),
                user_agent.as_deref(),
            )
            .await?;

        tracing::info!(
            ballot_id = %ballot.ballot_id,
            election_id = %req.election_id,
            voter_id = %voter_id,
            "Ballot cast successfully"
        );

        // Create vote commitment for blockchain (if enabled)
        let commitment_created = self
            .create_vote_commitment(tenant_id, req.election_id, ballot.ballot_id, &ballot_hash)
            .await
            .is_ok();

        Ok(CastBallotResponse {
            ballot_id: ballot.ballot_id,
            receipt_code: ballot.voter_receipt_code,
            ballot_hash: ballot.ballot_hash,
            cast_at: ballot.cast_at.to_rfc3339(),
            commitment_created,
        })
    }

    /// Verify receipt code
    pub async fn verify_receipt(&self, receipt_code: &str) -> Result<VerifyReceiptResponse> {
        let ballot = self
            .repository
            .get_ballot_by_receipt(receipt_code)
            .await?;

        match ballot {
            Some(b) => Ok(VerifyReceiptResponse {
                valid: true,
                ballot_id: Some(b.ballot_id),
                election_id: Some(b.election_id),
                cast_at: Some(b.cast_at.to_rfc3339()),
            }),
            None => Ok(VerifyReceiptResponse {
                valid: false,
                ballot_id: None,
                election_id: None,
                cast_at: None,
            }),
        }
    }

    /// Get voting status for user in election
    pub async fn get_voting_status(
        &self,
        election_id: ElectionId,
        voter_id: UserId,
    ) -> Result<VotingStatusResponse> {
        let ballot = self
            .repository
            .get_user_ballot(election_id, voter_id)
            .await?;

        match ballot {
            Some(b) => Ok(VotingStatusResponse {
                election_id,
                has_voted: true,
                ballot_id: Some(b.ballot_id),
                cast_at: Some(b.cast_at.to_rfc3339()),
            }),
            None => Ok(VotingStatusResponse {
                election_id,
                has_voted: false,
                ballot_id: None,
                cast_at: None,
            }),
        }
    }

    /// Validate votes
    async fn validate_votes(&self, req: &CastBallotRequest) -> Result<()> {
        if req.votes.is_empty() {
            return Err(AppError::ValidationError(
                "Ballot must contain at least one vote".to_string(),
            ));
        }

        // Get election positions
        let positions = sqlx::query!(
            "SELECT position_id, seats_available, max_votes_per_voter FROM positions WHERE election_id = $1",
            req.election_id.as_uuid()
        )
        .fetch_all(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get positions: {}", e)))?;

        if positions.is_empty() {
            return Err(AppError::ValidationError(
                "Election has no positions".to_string(),
            ));
        }

        // Validate each vote
        for vote in &req.votes {
            // Check position exists
            let position = positions
                .iter()
                .find(|p| p.position_id == vote.position_id)
                .ok_or_else(|| {
                    AppError::ValidationError(format!("Invalid position: {}", vote.position_id))
                })?;

            // Check vote count
            if !vote.is_abstain {
                let max_votes = position
                    .max_votes_per_voter
                    .unwrap_or(position.seats_available);

                if vote.candidate_ids.len() as i32 > max_votes {
                    return Err(AppError::ValidationError(format!(
                        "Too many candidates selected for position (max: {})",
                        max_votes
                    )));
                }

                if vote.candidate_ids.is_empty() {
                    return Err(AppError::ValidationError(
                        "Must select at least one candidate or abstain".to_string(),
                    ));
                }

                // Verify candidates exist and are approved
                for candidate_id in &vote.candidate_ids {
                    let candidate_exists = sqlx::query_scalar!(
                        "SELECT EXISTS(SELECT 1 FROM candidates WHERE candidate_id = $1 AND position_id = $2 AND is_approved = true AND withdrawn_at IS NULL)",
                        candidate_id,
                        vote.position_id
                    )
                    .fetch_one(self.db.pool())
                    .await
                    .map_err(|e| AppError::DatabaseError(format!("Failed to verify candidate: {}", e)))?
                    .unwrap_or(false);

                    if !candidate_exists {
                        return Err(AppError::ValidationError(format!(
                            "Invalid or unapproved candidate: {}",
                            candidate_id
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    /// Create vote commitment for blockchain
    async fn create_vote_commitment(
        &self,
        tenant_id: TenantId,
        election_id: ElectionId,
        ballot_id: uuid::Uuid,
        ballot_hash: &str,
    ) -> Result<()> {
        // Generate signing key pair (in production, use election-specific key)
        let key_pair = KeyPair::generate();

        // Sign the ballot hash
        let signature_data = sign_data(ballot_hash.as_bytes(), &key_pair)
            .map_err(|e| AppError::InternalError(format!("Signing failed: {}", e)))?;

        let signature_hex = hex::encode(&signature_data.signature);

        // Store commitment
        self.repository
            .store_commitment(tenant_id, election_id, ballot_id, ballot_hash, &signature_hex)
            .await?;

        tracing::info!(
            ballot_id = %ballot_id,
            commitment_hash = %ballot_hash,
            "Vote commitment created"
        );

        Ok(())
    }

    /// Get ballot count for election
    pub async fn get_ballot_count(&self, election_id: ElectionId) -> Result<i64> {
        self.repository.count_ballots(election_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_cast_ballot() {
        // Integration test placeholder
    }
}

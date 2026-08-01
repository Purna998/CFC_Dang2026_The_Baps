//! Result service - business logic layer

use eemp_crypto_service::{decrypt_ballot, deserialize_encrypted, EncryptionKey};
use eemp_database::Database;
use eemp_domain::{ElectionId, ElectionStatus, TenantId};
use eemp_error::{AppError, Result};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    calculator::{calculate_percentage, calculate_tallies, determine_winners},
    dto::*,
    models::DecryptedBallot,
    repository::ResultRepository,
};

pub struct ResultService {
    repository: ResultRepository,
    db: Database,
}

impl ResultService {
    pub fn new(db: Database) -> Self {
        Self {
            repository: ResultRepository::new(db.clone()),
            db,
        }
    }

    /// Calculate results for an election
    pub async fn calculate_results(&self, election_id: ElectionId) -> Result<()> {
        // Verify election is in Closed or Verifying status
        let election = sqlx::query!(
            "SELECT tenant_id, status FROM elections WHERE election_id = $1",
            election_id.as_uuid()
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get election: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Election not found".to_string()))?;

        let status: ElectionStatus = election
            .status
            .parse()
            .map_err(|e: String| AppError::InternalError(e))?;

        if status != ElectionStatus::Closed && status != ElectionStatus::Verifying {
            return Err(AppError::ValidationError(format!(
                "Cannot calculate results for election in {:?} status",
                status
            )));
        }

        let tenant_id = TenantId::from_uuid(election.tenant_id);

        tracing::info!(
            election_id = %election_id,
            "Starting result calculation"
        );

        // Delete existing results (for recalculation)
        self.repository.delete_election_results(election_id).await?;

        // Get encrypted ballots
        let encrypted_ballots = self.repository.get_encrypted_ballots(election_id).await?;

        if encrypted_ballots.is_empty() {
            tracing::warn!(election_id = %election_id, "No ballots to count");
            return Ok(());
        }

        // Decrypt ballots (in production, use proper key management)
        // For MVP, we generate a temporary key (in production, retrieve from secure storage)
        let decryption_key = self.get_decryption_key(encrypted_ballots[0].2).await?;

        let mut decrypted_ballots = Vec::new();
        for (ballot_id, encrypted_str, _key_id) in encrypted_ballots {
            match self.decrypt_single_ballot(ballot_id, &encrypted_str, &decryption_key) {
                Ok(ballot) => decrypted_ballots.push(ballot),
                Err(e) => {
                    tracing::warn!(
                        ballot_id = %ballot_id,
                        error = %e,
                        "Failed to decrypt ballot, skipping"
                    );
                    continue;
                }
            }
        }

        tracing::info!(
            election_id = %election_id,
            total_ballots = decrypted_ballots.len(),
            "Ballots decrypted"
        );

        // Calculate tallies
        let tallies = calculate_tallies(decrypted_ballots)?;

        // Get position details
        let positions = self.repository.get_positions(election_id).await?;

        // Store results for each position
        for (position_id, _title, seats_available) in positions {
            if let Some(tally) = tallies.get(&position_id) {
                // Determine winners
                let results = determine_winners(&tally.candidate_votes, seats_available);

                for (candidate_id, vote_count, rank) in results {
                    let percentage = calculate_percentage(vote_count, tally.total_votes);

                    self.repository
                        .store_result(
                            tenant_id,
                            election_id,
                            position_id,
                            Some(candidate_id),
                            vote_count,
                            percentage,
                            rank <= seats_available,
                            rank,
                        )
                        .await?;
                }

                // Store abstain count if any
                if tally.abstain_count > 0 {
                    let percentage = calculate_percentage(tally.abstain_count, tally.total_votes);
                    self.repository
                        .store_result(
                            tenant_id,
                            election_id,
                            position_id,
                            None, // No candidate for abstain
                            tally.abstain_count,
                            percentage,
                            false,
                            999, // High rank for abstain
                        )
                        .await?;
                }
            }
        }

        tracing::info!(
            election_id = %election_id,
            "Result calculation completed"
        );

        Ok(())
    }

    /// Publish results
    pub async fn publish_results(&self, election_id: ElectionId) -> Result<()> {
        // Verify election is in Published or Verifying status
        let status = sqlx::query_scalar!(
            "SELECT status FROM elections WHERE election_id = $1",
            election_id.as_uuid()
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get election: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Election not found".to_string()))?;

        let election_status: ElectionStatus = status
            .parse()
            .map_err(|e: String| AppError::InternalError(e))?;

        if election_status != ElectionStatus::Verifying
            && election_status != ElectionStatus::Published
        {
            return Err(AppError::ValidationError(format!(
                "Cannot publish results for election in {:?} status",
                election_status
            )));
        }

        // Mark results as published
        self.repository.publish_results(election_id).await?;

        tracing::info!(election_id = %election_id, "Results published");

        Ok(())
    }

    /// Get election results
    pub async fn get_results(&self, election_id: ElectionId) -> Result<ElectionResultsResponse> {
        let results = self.repository.get_election_results(election_id).await?;

        if results.is_empty() {
            return Err(AppError::NotFound(
                "No results found for this election".to_string(),
            ));
        }

        // Get positions and candidates
        let positions = self.repository.get_positions(election_id).await?;
        let candidates = self.repository.get_candidates(election_id).await?;

        // Build candidate map
        let candidate_map: HashMap<Uuid, (Uuid, String)> = candidates
            .into_iter()
            .map(|(cid, pid, name)| (cid, (pid, name)))
            .collect();

        // Group results by position
        let mut position_results: HashMap<Uuid, Vec<_>> = HashMap::new();
        for result in results {
            position_results
                .entry(result.position_id)
                .or_insert_with(Vec::new)
                .push(result);
        }

        // Build response
        let position_responses: Vec<PositionResultResponse> = positions
            .into_iter()
            .filter_map(|(pos_id, title, seats)| {
                position_results.get(&pos_id).map(|results| {
                    let total_votes: i32 = results.iter().map(|r| r.vote_count).sum();

                    let candidates: Vec<CandidateResultResponse> = results
                        .iter()
                        .filter_map(|r| {
                            r.candidate_id.and_then(|cid| {
                                candidate_map.get(&cid).map(|(_, name)| CandidateResultResponse {
                                    candidate_id: cid,
                                    candidate_name: name.clone(),
                                    vote_count: r.vote_count,
                                    vote_percentage: r.vote_percentage.unwrap_or(0.0),
                                    is_winner: r.is_winner,
                                    rank: r.rank.unwrap_or(999),
                                })
                            })
                        })
                        .collect();

                    PositionResultResponse {
                        position_id: pos_id,
                        position_title: title,
                        seats_available: seats,
                        total_votes,
                        candidates,
                    }
                })
            })
            .collect();

        let total_ballots = position_responses
            .first()
            .map(|p| p.total_votes)
            .unwrap_or(0);

        let calculated_at = position_results
            .values()
            .flat_map(|v| v.iter())
            .map(|r| r.calculated_at)
            .max()
            .unwrap_or_else(chrono::Utc::now);

        let published_at = position_results
            .values()
            .flat_map(|v| v.iter())
            .filter_map(|r| r.published_at)
            .max();

        Ok(ElectionResultsResponse {
            election_id,
            total_ballots,
            positions: position_responses,
            calculated_at: calculated_at.to_rfc3339(),
            published_at: published_at.map(|dt| dt.to_rfc3339()),
        })
    }

    /// Decrypt a single ballot
    fn decrypt_single_ballot(
        &self,
        ballot_id: Uuid,
        encrypted_str: &str,
        key: &EncryptionKey,
    ) -> Result<DecryptedBallot> {
        let encrypted = deserialize_encrypted(encrypted_str)
            .map_err(|e| AppError::InternalError(format!("Failed to deserialize ballot: {}", e)))?;

        let decrypted_bytes = decrypt_ballot(&encrypted, key.as_bytes())
            .map_err(|e| AppError::InternalError(format!("Failed to decrypt ballot: {}", e)))?;

        let ballot_content: crate::models::DecryptedBallot =
            serde_json::from_slice(&decrypted_bytes).map_err(|e| {
                AppError::InternalError(format!("Failed to parse ballot JSON: {}", e))
            })?;

        Ok(DecryptedBallot {
            ballot_id,
            election_id: ballot_content.election_id,
            votes: ballot_content.votes,
        })
    }

    /// Get decryption key (in production, retrieve from secure key management)
    async fn get_decryption_key(&self, _key_id: Uuid) -> Result<EncryptionKey> {
        // For MVP, generate a temporary key
        // In production, retrieve from secure storage (HashiCorp Vault, AWS KMS, etc.)
        Ok(eemp_crypto_service::generate_encryption_key())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_calculate_results() {
        // Integration test placeholder
    }
}

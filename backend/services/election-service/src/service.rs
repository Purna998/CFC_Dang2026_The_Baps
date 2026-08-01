//! Election service - business logic layer

use eemp_database::Database;
use eemp_domain::{ElectionId, ElectionStatus, TenantId, UserId};
use eemp_error::{AppError, Result};
use uuid::Uuid;

use crate::{
    dto::*,
    repository::ElectionRepository,
    state_machine::ElectionStateMachine,
};

pub struct ElectionService {
    repository: ElectionRepository,
}

impl ElectionService {
    pub fn new(db: Database) -> Self {
        Self {
            repository: ElectionRepository::new(db),
        }
    }

    /// Create a new election
    pub async fn create_election(
        &self,
        tenant_id: TenantId,
        created_by: UserId,
        req: CreateElectionRequest,
    ) -> Result<CreateElectionResponse> {
        // Validate dates
        if req.voting_end_time <= req.voting_start_time {
            return Err(AppError::ValidationError(
                "voting_end_time must be after voting_start_time".to_string(),
            ));
        }

        if let Some(publish_time) = req.result_publish_time {
            if publish_time < req.voting_end_time {
                return Err(AppError::ValidationError(
                    "result_publish_time cannot be before voting_end_time".to_string(),
                ));
            }
        }

        let election = self
            .repository
            .create_election(
                tenant_id,
                &req.title,
                req.description.as_deref(),
                req.election_type,
                req.voting_start_time,
                req.voting_end_time,
                req.result_publish_time,
                req.allow_write_in_candidates,
                req.allow_abstain,
                req.require_identity_verification,
                req.enable_blockchain_verification,
                req.max_votes_per_voter,
                created_by,
            )
            .await?;

        tracing::info!(
            election_id = %election.election_id,
            title = %election.title,
            "Election created"
        );

        Ok(CreateElectionResponse {
            election_id: election.election_id,
            title: election.title,
            status: election.status,
        })
    }

    /// Get election by ID
    pub async fn get_election(&self, election_id: ElectionId) -> Result<ElectionResponse> {
        let election = self
            .repository
            .get_by_id(election_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Election not found".to_string()))?;

        Ok(ElectionResponse::from(election))
    }

    /// Transition election to new status
    pub async fn transition_status(
        &self,
        election_id: ElectionId,
        new_status: ElectionStatus,
    ) -> Result<ElectionResponse> {
        // Get current election
        let election = self
            .repository
            .get_by_id(election_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Election not found".to_string()))?;

        // Validate state transition
        ElectionStateMachine::validate_transition(&election.status, &new_status)?;

        // Additional business logic validation
        match new_status {
            ElectionStatus::Scheduled => {
                // Verify election has at least one position
                let positions = self.repository.list_positions(election_id).await?;
                if positions.is_empty() {
                    return Err(AppError::ValidationError(
                        "Cannot schedule election without positions".to_string(),
                    ));
                }
            }
            ElectionStatus::Open => {
                // Verify we're at or past voting start time
                if chrono::Utc::now() < election.voting_start_time {
                    return Err(AppError::ValidationError(
                        "Cannot open election before voting_start_time".to_string(),
                    ));
                }
            }
            ElectionStatus::Closed => {
                // Verify we're at or past voting end time
                if chrono::Utc::now() < election.voting_end_time {
                    return Err(AppError::ValidationError(
                        "Cannot close election before voting_end_time".to_string(),
                    ));
                }
            }
            _ => {}
        }

        // Update status
        self.repository
            .update_status(election_id, new_status.clone())
            .await?;

        tracing::info!(
            election_id = %election_id,
            old_status = ?election.status,
            new_status = ?new_status,
            "Election status transitioned"
        );

        // Return updated election
        self.get_election(election_id).await
    }

    /// List elections for tenant
    pub async fn list_elections(
        &self,
        tenant_id: TenantId,
        limit: i64,
        offset: i64,
    ) -> Result<ElectionListResponse> {
        let (elections, total) = self.repository.list_by_tenant(tenant_id, limit, offset).await?;

        Ok(ElectionListResponse {
            elections: elections.into_iter().map(ElectionResponse::from).collect(),
            total,
        })
    }

    /// Create a position for an election
    pub async fn create_position(
        &self,
        tenant_id: TenantId,
        election_id: ElectionId,
        req: CreatePositionRequest,
    ) -> Result<PositionResponse> {
        // Verify election exists and is modifiable
        let election = self
            .repository
            .get_by_id(election_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Election not found".to_string()))?;

        if !ElectionStateMachine::is_modifiable(&election.status) {
            return Err(AppError::ValidationError(format!(
                "Cannot modify positions for election in {:?} status",
                election.status
            )));
        }

        let position = self
            .repository
            .create_position(
                tenant_id,
                election_id,
                &req.title,
                req.description.as_deref(),
                req.display_order,
                req.seats_available,
                req.min_votes_required,
                req.max_votes_per_voter,
            )
            .await?;

        tracing::info!(
            position_id = %position.position_id,
            election_id = %election_id,
            title = %position.title,
            "Position created"
        );

        Ok(PositionResponse::from(position))
    }

    /// List positions for an election
    pub async fn list_positions(&self, election_id: ElectionId) -> Result<Vec<PositionResponse>> {
        let positions = self.repository.list_positions(election_id).await?;
        Ok(positions.into_iter().map(PositionResponse::from).collect())
    }

    /// Get allowed next states for election
    pub fn get_allowed_transitions(&self, current_status: ElectionStatus) -> Vec<ElectionStatus> {
        ElectionStateMachine::allowed_next_states(&current_status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_create_election() {
        // Integration test placeholder
    }
}

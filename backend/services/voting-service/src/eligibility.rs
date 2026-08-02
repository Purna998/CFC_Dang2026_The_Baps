//! Voter eligibility verification

use eemp_database::Database;
use eemp_domain::{ElectionId, UserId};
use eemp_error::{AppError, Result};

/// Check if user is eligible to vote in an election
pub async fn verify_voter_eligibility(
    db: &Database,
    election_id: ElectionId,
    voter_id: UserId,
) -> Result<bool> {
    // Check if election is open
    let election_status = sqlx::query_scalar!(
        "SELECT status FROM elections WHERE election_id = $1",
        election_id.as_uuid()
    )
    .fetch_optional(db.pool())
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to get election status: {}", e)))?
    .ok_or_else(|| AppError::ResourceNotFound("Election not found".to_string()))?;

    if election_status != "Open" {
        return Err(AppError::ValidationError(
            "Election is not open for voting".to_string(),
        ));
    }

    // Check if user has already voted
    let has_voted = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM ballots WHERE election_id = $1 AND voter_id = $2)",
        election_id.as_uuid(),
        voter_id.as_uuid()
    )
    .fetch_one(db.pool())
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to check voting status: {}", e)))?
    .unwrap_or(false);

    if has_voted {
        return Err(AppError::AlreadyVoted);
    }

    // Check eligibility rules (simplified - in production, check against eligibility_rules table)
    // For MVP, all active users are eligible
    let user_is_active = sqlx::query_scalar!(
        "SELECT is_active FROM users WHERE user_id = $1",
        voter_id.as_uuid()
    )
    .fetch_optional(db.pool())
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to get user status: {}", e)))?
    .ok_or_else(|| AppError::ResourceNotFound("User not found".to_string()))?;

    if !user_is_active {
        return Err(AppError::InsufficientPermissions(
            "User account is not active".to_string(),
        ));
    }

    Ok(true)
}

/// Verify voting time window
pub async fn verify_voting_time(db: &Database, election_id: ElectionId) -> Result<bool> {
    let election = sqlx::query!(
        "SELECT voting_start_time, voting_end_time FROM elections WHERE election_id = $1",
        election_id.as_uuid()
    )
    .fetch_optional(db.pool())
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to get election: {}", e)))?
    .ok_or_else(|| AppError::ResourceNotFound("Election not found".to_string()))?;

    let now = chrono::Utc::now();
    let start = election.voting_start_time;
    let end = election.voting_end_time;

    if now < start {
        return Err(AppError::ValidationError(
            "Voting has not started yet".to_string(),
        ));
    }

    if now > end {
        return Err(AppError::ValidationError(
            "Voting has ended".to_string(),
        ));
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_verify_eligibility() {
        // Integration test placeholder
    }
}

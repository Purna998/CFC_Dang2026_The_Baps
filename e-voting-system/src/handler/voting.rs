use axum::{extract::{Path, State}, Extension, Json};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{Ballot, CastVoteRequest, Claims, SuccessResponse},
    services::{voter_service, voting_service},
    AppState,
};

#[derive(serde::Serialize)]
pub struct VoteResponse {
    pub verification_code: String,
    pub message: String,
}

pub async fn cast_vote(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CastVoteRequest>,
) -> Result<Json<SuccessResponse<VoteResponse>>, AppError> {
    // Ensure voter role
    if claims.role != "voter" {
        return Err(AppError::Forbidden("Only voters can cast votes".to_string()));
    }

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // Get voter ID from user ID
    let voter = voter_service::get_voter_by_user_id(&state.db, user_id).await?;

    let verification_code = voting_service::cast_vote(
        &state.db,
        payload,
        voter.id,
        &state.config.jwt_secret,
    )
    .await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: VoteResponse {
            verification_code: verification_code.clone(),
            message: format!(
                "Vote cast successfully. Save this verification code: {}",
                verification_code
            ),
        },
    }))
}

pub async fn verify_vote(
    State(state): State<AppState>,
    Path(verification_code): Path<String>,
) -> Result<Json<SuccessResponse<Ballot>>, AppError> {
    let ballot = voting_service::verify_vote(&state.db, &verification_code).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: ballot,
    }))
}

#[derive(serde::Serialize)]
pub struct VoteStatusResponse {
    pub has_voted: bool,
}

pub async fn check_vote_status(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(election_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<VoteStatusResponse>>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let voter = voter_service::get_voter_by_user_id(&state.db, user_id).await?;

    let has_voted = voting_service::check_vote_status(&state.db, election_id, voter.id).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: VoteStatusResponse { has_voted },
    }))
}

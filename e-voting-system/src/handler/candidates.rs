use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    models::{Candidate, CreateCandidateRequest, MessageResponse, SuccessResponse, UpdateCandidateRequest},
    services::candidate_service,
    AppState,
};

pub async fn create_candidate(
    State(state): State<AppState>,
    Json(payload): Json<CreateCandidateRequest>,
) -> Result<Json<SuccessResponse<Candidate>>, AppError> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let candidate = candidate_service::create_candidate(&state.db, payload).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: candidate,
    }))
}

pub async fn list_candidates(
    State(state): State<AppState>,
    Path(election_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<Vec<Candidate>>>, AppError> {
    let candidates = candidate_service::list_candidates_by_election(&state.db, election_id).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: candidates,
    }))
}

pub async fn update_candidate(
    State(state): State<AppState>,
    Path(candidate_id): Path<Uuid>,
    Json(payload): Json<UpdateCandidateRequest>,
) -> Result<Json<SuccessResponse<Candidate>>, AppError> {
    let candidate = candidate_service::update_candidate(&state.db, candidate_id, payload).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: candidate,
    }))
}

pub async fn delete_candidate(
    State(state): State<AppState>,
    Path(candidate_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    candidate_service::delete_candidate(&state.db, candidate_id).await?;

    Ok(Json(MessageResponse {
        message: "Candidate deleted successfully".to_string(),
    }))
}

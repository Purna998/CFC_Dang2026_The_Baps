use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    models::{MessageResponse, RegisterVoterRequest, SuccessResponse, User, Voter},
    services::voter_service,
    AppState,
};

pub async fn register_voter(
    State(state): State<AppState>,
    Json(payload): Json<RegisterVoterRequest>,
) -> Result<Json<SuccessResponse<Voter>>, AppError> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let voter = voter_service::register_voter(&state.db, payload).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: voter,
    }))
}

pub async fn list_voters(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Vec<(User, Voter)>>>, AppError> {
    let voters = voter_service::list_all_voters(&state.db).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: voters,
    }))
}

pub async fn verify_voter(
    State(state): State<AppState>,
    Path(voter_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    voter_service::verify_voter(&state.db, voter_id).await?;

    Ok(Json(MessageResponse {
        message: "Voter verified successfully".to_string(),
    }))
}

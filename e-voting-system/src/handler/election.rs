use axum::{extract::{Path, State}, Extension, Json};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    models::{Claims, CreateElectionRequest, Election, ElectionStatus, MessageResponse, SuccessResponse, UpdateElectionRequest},
    services::election_service,
    AppState,
};

pub async fn create_election(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateElectionRequest>,
) -> Result<Json<SuccessResponse<Election>>, AppError> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let election = election_service::create_election(&state.db, payload, user_id).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: election,
    }))
}

pub async fn get_election(
    State(state): State<AppState>,
    Path(election_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<Election>>, AppError> {
    let election = election_service::get_election_by_id(&state.db, election_id).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: election,
    }))
}

pub async fn list_elections(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SuccessResponse<Vec<Election>>>, AppError> {
    let include_drafts = claims.role == "admin";
    let elections = election_service::list_elections(&state.db, include_drafts).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: elections,
    }))
}

pub async fn update_election(
    State(state): State<AppState>,
    Path(election_id): Path<Uuid>,
    Json(payload): Json<UpdateElectionRequest>,
) -> Result<Json<SuccessResponse<Election>>, AppError> {
    let election = election_service::update_election(&state.db, election_id, payload).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: election,
    }))
}

pub async fn open_election(
    State(state): State<AppState>,
    Path(election_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    election_service::change_election_status(&state.db, election_id, ElectionStatus::Open).await?;

    Ok(Json(MessageResponse {
        message: "Election opened successfully".to_string(),
    }))
}

pub async fn close_election(
    State(state): State<AppState>,
    Path(election_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    election_service::change_election_status(&state.db, election_id, ElectionStatus::Closed).await?;

    Ok(Json(MessageResponse {
        message: "Election closed successfully".to_string(),
    }))
}

pub async fn archive_election(
    State(state): State<AppState>,
    Path(election_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    election_service::change_election_status(&state.db, election_id, ElectionStatus::Archived).await?;

    Ok(Json(MessageResponse {
        message: "Election archived successfully".to_string(),
    }))
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    models::{MessageResponse, Party, RegisterPartyRequest, SuccessResponse, PartyCreationResponse},
    services::party_service,
    AppState,
};

// List all parties
pub async fn list_parties(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Vec<Party>>>, AppError> {
    let parties = party_service::list_all_parties(&state.db).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: parties,
    }))
}

// Get party by ID
pub async fn get_party(
    State(state): State<AppState>,
    Path(party_id): Path<Uuid>,
) -> Result<Json<SuccessResponse<Party>>, AppError> {
    let party = party_service::get_party_by_id(&state.db, party_id).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: party,
    }))
}

// Create new party
pub async fn create_party(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPartyRequest>,
) -> Result<Json<SuccessResponse<PartyCreationResponse>>, AppError> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let (party, temporary_password) = party_service::create_party(&state.db, payload).await?;

    let email = party.contact_email.clone().unwrap_or_else(|| "the registered email".to_string());

    Ok(Json(SuccessResponse {
        success: true,
        data: PartyCreationResponse {
            party,
            temporary_password,
            message: format!(
                "Party created successfully. Login credentials have been sent to {} (Password shown once - save it now)",
                email
            ),
        },
    }))
}

// Update party
pub async fn update_party(
    State(state): State<AppState>,
    Path(party_id): Path<Uuid>,
    Json(payload): Json<RegisterPartyRequest>,
) -> Result<Json<SuccessResponse<Party>>, AppError> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let party = party_service::update_party(&state.db, party_id, payload).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: party,
    }))
}

// Delete party
pub async fn delete_party(
    State(state): State<AppState>,
    Path(party_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    party_service::delete_party(&state.db, party_id).await?;

    Ok(Json(MessageResponse {
        message: "Party deleted successfully".to_string(),
    }))
}

// Verify party (Admin only)
pub async fn verify_party(
    State(state): State<AppState>,
    Path(party_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, AppError> {
    party_service::verify_party(&state.db, party_id).await?;

    Ok(Json(MessageResponse {
        message: "Party verified successfully".to_string(),
    }))
}

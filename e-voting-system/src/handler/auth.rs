use axum::{extract::State, Json};
use validator::Validate;

use crate::{
    error::AppError,
    models::{LoginRequest, LoginResponse, SuccessResponse},
    services::auth_service,
    AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<SuccessResponse<LoginResponse>>, AppError> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let response = auth_service::authenticate_user(
        &state.db,
        payload,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )
    .await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: response,
    }))
}

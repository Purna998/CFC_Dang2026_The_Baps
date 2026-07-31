use axum::{extract::State, Extension, Json};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    models::{ChangePasswordRequest, Claims, MessageResponse, SuccessResponse, UserResponse},
    services::auth_service,
    utils::{hash_password, verify_password},
    AppState,
};

pub async fn get_profile(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SuccessResponse<UserResponse>>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let user = auth_service::get_user_by_id(&state.db, user_id).await?;

    Ok(Json(SuccessResponse {
        success: true,
        data: UserResponse {
            id: user.id,
            email: user.email,
            full_name: user.full_name,
            role: user.role,
        },
    }))
}

pub async fn change_password(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let user = auth_service::get_user_by_id(&state.db, user_id).await?;

    // Verify old password
    if !verify_password(&payload.old_password, &user.password_hash)? {
        return Err(AppError::BadRequest("Incorrect old password".to_string()));
    }

    // Hash new password
    let new_hash = hash_password(&payload.new_password)?;

    // Update password
    sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2")
        .bind(&new_hash)
        .bind(user_id)
        .execute(&state.db)
        .await?;

    Ok(Json(MessageResponse {
        message: "Password changed successfully".to_string(),
    }))
}

//! Authentication middleware

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use eemp_domain::UserRole;
use eemp_error::AppError;
use std::str::FromStr;

use crate::{extractors::AuthUser, state::AppState};

/// Authentication middleware
///
/// Validates JWT token and extracts user information.
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

    // Expect "Bearer <token>"
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::Unauthorized("Invalid Authorization header format".to_string()))?;

    // Verify token (this would use AuthService in a real implementation)
    // For now, we'll create a placeholder that uses the JWT manager
    let jwt_manager = eemp_auth_service::jwt::JwtManager::new(state.config.jwt.clone())?;
    let claims = jwt_manager.verify_access_token(token)?;

    // Parse claims
    let user_id = eemp_domain::UserId::from_str(&claims.sub)
        .map_err(|e| AppError::Unauthorized(format!("Invalid user ID: {}", e)))?;

    let tenant_id = eemp_domain::TenantId::from_str(&claims.tenant_id)
        .map_err(|e| AppError::Unauthorized(format!("Invalid tenant ID: {}", e)))?;

    let role = UserRole::from_str(&claims.role)
        .map_err(|e| AppError::Unauthorized(format!("Invalid role: {}", e)))?;

    // Create AuthUser and insert into request extensions
    let auth_user = AuthUser {
        user_id,
        tenant_id,
        role,
    };

    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

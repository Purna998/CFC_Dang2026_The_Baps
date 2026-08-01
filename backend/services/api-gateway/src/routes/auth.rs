//! Authentication route handlers

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use eemp_auth_service::dto::*;
use eemp_error::Result;

use crate::{
    extractors::{AuthUser, ClientIp, UserAgent, ValidatedJson},
    middleware::auth_middleware,
    state::AppState,
};

/// Create authentication routes
pub fn auth_routes() -> Router<AppState> {
    Router::new()
        // Public routes (no authentication required)
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/refresh", post(refresh_token))
        // Protected routes (require authentication)
        .route("/logout", post(logout))
        .route("/me", get(get_current_user))
        .route("/mfa/enable", post(enable_mfa))
        .route("/mfa/verify", post(verify_mfa))
}

/// Login handler
///
/// POST /api/v1/auth/login
async fn login(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    UserAgent(user_agent): UserAgent,
    ValidatedJson(req): ValidatedJson<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let response = state.auth_service.login(req, ip, user_agent).await?;
    Ok(Json(response))
}

/// Register handler
///
/// POST /api/v1/auth/register
async fn register(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<RegisterRequest>,
) -> Result<Json<RegisterResponse>> {
    let response = state.auth_service.register(req).await?;
    Ok(Json(response))
}

/// Refresh token handler
///
/// POST /api/v1/auth/refresh
async fn refresh_token(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>> {
    let response = state.auth_service.refresh_token(req).await?;
    Ok(Json(response))
}

/// Logout handler
///
/// POST /api/v1/auth/logout
async fn logout(
    State(state): State<AppState>,
    Json(req): Json<LogoutRequest>,
) -> Result<Json<()>> {
    state.auth_service.logout(req).await?;
    Ok(Json(()))
}

/// Get current user handler
///
/// GET /api/v1/auth/me
async fn get_current_user(auth_user: AuthUser) -> Result<Json<UserInfo>> {
    Ok(Json(UserInfo {
        user_id: auth_user.user_id,
        tenant_id: auth_user.tenant_id,
        email: String::new(), // TODO: Fetch from database
        role: auth_user.role.to_string(),
    }))
}

/// Enable MFA handler
///
/// POST /api/v1/auth/mfa/enable
async fn enable_mfa(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<EnableMfaResponse>> {
    let email = "user@example.com"; // TODO: Fetch from database
    let response = state
        .auth_service
        .enable_mfa(auth_user.user_id, email)
        .await?;
    Ok(Json(response))
}

/// Verify MFA handler
///
/// POST /api/v1/auth/mfa/verify
async fn verify_mfa(
    State(state): State<AppState>,
    auth_user: AuthUser,
    ValidatedJson(req): ValidatedJson<VerifyMfaRequest>,
) -> Result<Json<VerifyMfaResponse>> {
    let email = "user@example.com"; // TODO: Fetch from database
    let response = state
        .auth_service
        .verify_mfa(auth_user.user_id, email, &req.totp_code)
        .await?;
    Ok(Json(response))
}

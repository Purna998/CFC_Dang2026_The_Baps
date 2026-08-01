//! Tenant resolution middleware

use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use eemp_error::{AppError, Result};

use crate::{extractors::AuthUser, state::AppState};

/// Tenant resolution middleware
///
/// Sets the tenant context for database queries using Row-Level Security.
pub async fn tenant_middleware(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Result<Response> {
    // Extract tenant ID from authenticated user (set by auth middleware)
    if let Some(auth_user) = request.extensions().get::<AuthUser>() {
        // Set tenant context for this request
        state.db.set_tenant_context(auth_user.tenant_id).await?;
    }
    // If no auth user, skip tenant context (e.g., public endpoints)

    Ok(next.run(request).await)
}

use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum::http::header;

use crate::{models::Claims, utils::jwt::verify_token, AppState};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for public routes
    let path = req.uri().path();
    if path == "/health" || path == "/api/auth/login" {
        return Ok(next.run(req).await);
    }

    // Extract token from Authorization header
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_string())
            } else {
                None
            }
        });

    let token = match token {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Verify token
    let claims = verify_token(&token, &state.config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check role-based access for admin routes
    if path.starts_with("/api/admin") && claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Add claims to request extensions
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

// Extension trait to extract claims from request
pub trait ClaimsExt {
    fn claims(&self) -> Option<&Claims>;
}

impl ClaimsExt for axum::http::Extensions {
    fn claims(&self) -> Option<&Claims> {
        self.get::<Claims>()
    }
}

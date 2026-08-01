//! Custom extractors for Axum handlers

use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use eemp_domain::{TenantId, UserId, UserRole};
use eemp_error::{AppError, ErrorResponse};
use serde::Deserialize;
use validator::Validate;

/// Validated JSON extractor
///
/// Automatically validates the request body using the validator crate.
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: for<'de> Deserialize<'de> + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum::Json(value) = axum::Json::<T>::from_request(req, state)
            .await
            .map_err(|e| {
                let error_response = ErrorResponse::new(
                    "validation_error",
                    "Invalid JSON",
                    Some(e.to_string()),
                    None,
                );
                (StatusCode::BAD_REQUEST, axum::Json(error_response)).into_response()
            })?;

        value.validate().map_err(|e| {
            let error_response = ErrorResponse::new(
                "validation_error",
                "Validation failed",
                Some(e.to_string()),
                None,
            );
            (StatusCode::UNPROCESSABLE_ENTITY, axum::Json(error_response)).into_response()
        })?;

        Ok(ValidatedJson(value))
    }
}

/// Authenticated user extracted from JWT token
pub struct AuthUser {
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub role: UserRole,
}

#[async_trait]
impl<S> FromRequest<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract user from request extensions (set by auth middleware)
        req.extensions()
            .get::<AuthUser>()
            .cloned()
            .ok_or_else(|| {
                AppError::Unauthorized("Authentication required".to_string()).into_response()
            })
    }
}

impl Clone for AuthUser {
    fn clone(&self) -> Self {
        Self {
            user_id: self.user_id,
            tenant_id: self.tenant_id,
            role: self.role,
        }
    }
}

/// Client IP address
pub struct ClientIp(pub Option<String>);

#[async_trait]
impl<S> FromRequest<S> for ClientIp
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        // Try X-Forwarded-For first (behind proxy)
        let ip = req
            .headers()
            .get("X-Forwarded-For")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split(',').next())
            .map(|s| s.trim().to_string())
            .or_else(|| {
                // Fallback to X-Real-IP
                req.headers()
                    .get("X-Real-IP")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string())
            });

        Ok(ClientIp(ip))
    }
}

/// User agent string
pub struct UserAgent(pub Option<String>);

#[async_trait]
impl<S> FromRequest<S> for UserAgent
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let user_agent = req
            .headers()
            .get("User-Agent")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Ok(UserAgent(user_agent))
    }
}

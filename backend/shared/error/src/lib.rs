//! Centralized error handling for EEMP
//!
//! This library provides:
//! - Domain errors (business logic errors)
//! - API errors (HTTP error responses)
//! - Error conversion traits
//! - Standardized error response format

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Standard API error response format
///
/// Following the API design specification:
/// ```json
/// {
///   "error": {
///     "code": "validation_error",
///     "message": "Validation failed",
///     "details": "Additional context",
///     "field": "email",
///     "request_id": "uuid"
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    pub request_id: String,
}

impl ErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: ErrorDetail {
                code: code.into(),
                message: message.into(),
                details: None,
                field: None,
                request_id: uuid::Uuid::new_v4().to_string(),
            },
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.error.details = Some(details.into());
        self
    }

    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.error.field = Some(field.into());
        self
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.error.request_id = request_id.into();
        self
    }
}

/// Application error types
///
/// Following the error codes defined in the API specification.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // Authentication Errors (401)
    #[error("Missing authorization token")]
    MissingToken,

    #[error("Invalid authorization token")]
    InvalidToken,

    #[error("Token has expired")]
    TokenExpired,

    #[error("Token has been revoked")]
    TokenRevoked,

    // Authorization Errors (403)
    #[error("Insufficient permissions: {0}")]
    InsufficientPermissions(String),

    #[error("Tenant mismatch")]
    TenantMismatch,

    // Validation Errors (400, 422)
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Invalid format for field: {0}")]
    InvalidFormat(String),

    #[error("Required field missing: {0}")]
    RequiredField(String),

    #[error("Field too long: {0}")]
    FieldTooLong(String),

    #[error("Field too short: {0}")]
    FieldTooShort(String),

    #[error("Invalid enum value: {0}")]
    InvalidEnumValue(String),

    // Resource Errors (404, 409)
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Duplicate resource: {0}")]
    DuplicateResource(String),

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    // Business Logic Errors (422)
    #[error("Election has already started")]
    ElectionAlreadyStarted,

    #[error("User has already voted in this election")]
    AlreadyVoted,

    #[error("User is not eligible to vote in this election")]
    NotEligible,

    #[error("Election is not open for voting")]
    ElectionNotOpen,

    // Rate Limiting (429)
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    // Server Errors (500, 503)
    #[error("Internal server error")]
    InternalError,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Blockchain error: {0}")]
    BlockchainError(String),

    #[error("Service unavailable")]
    ServiceUnavailable,

    // External errors
    #[error("External error: {0}")]
    External(String),
}

impl AppError {
    /// Convert to HTTP status code
    pub fn status_code(&self) -> StatusCode {
        match self {
            // 401 Unauthorized
            Self::MissingToken
            | Self::InvalidToken
            | Self::TokenExpired
            | Self::TokenRevoked => StatusCode::UNAUTHORIZED,

            // 403 Forbidden
            Self::InsufficientPermissions(_) | Self::TenantMismatch => StatusCode::FORBIDDEN,

            // 400 Bad Request
            Self::ValidationError(_)
            | Self::InvalidFormat(_)
            | Self::RequiredField(_)
            | Self::FieldTooLong(_)
            | Self::FieldTooShort(_)
            | Self::InvalidEnumValue(_) => StatusCode::BAD_REQUEST,

            // 404 Not Found
            Self::ResourceNotFound(_) => StatusCode::NOT_FOUND,

            // 409 Conflict
            Self::DuplicateResource(_) | Self::ConstraintViolation(_) => StatusCode::CONFLICT,

            // 422 Unprocessable Entity
            Self::ElectionAlreadyStarted
            | Self::AlreadyVoted
            | Self::NotEligible
            | Self::ElectionNotOpen => StatusCode::UNPROCESSABLE_ENTITY,

            // 429 Too Many Requests
            Self::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,

            // 500 Internal Server Error
            Self::InternalError | Self::DatabaseError(_) | Self::External(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }

            // 503 Service Unavailable
            Self::ServiceUnavailable | Self::BlockchainError(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// Convert to error code (machine-readable)
    pub fn error_code(&self) -> &'static str {
        match self {
            // Authentication
            Self::MissingToken => "missing_token",
            Self::InvalidToken => "invalid_token",
            Self::TokenExpired => "token_expired",
            Self::TokenRevoked => "token_revoked",

            // Authorization
            Self::InsufficientPermissions(_) => "insufficient_permissions",
            Self::TenantMismatch => "tenant_mismatch",

            // Validation
            Self::ValidationError(_) => "validation_error",
            Self::InvalidFormat(_) => "invalid_format",
            Self::RequiredField(_) => "required_field",
            Self::FieldTooLong(_) => "field_too_long",
            Self::FieldTooShort(_) => "field_too_short",
            Self::InvalidEnumValue(_) => "invalid_enum_value",

            // Resources
            Self::ResourceNotFound(_) => "resource_not_found",
            Self::DuplicateResource(_) => "duplicate_resource",
            Self::ConstraintViolation(_) => "constraint_violation",

            // Business Logic
            Self::ElectionAlreadyStarted => "election_already_started",
            Self::AlreadyVoted => "already_voted",
            Self::NotEligible => "not_eligible",
            Self::ElectionNotOpen => "election_not_open",

            // Rate Limiting
            Self::RateLimitExceeded => "rate_limit_exceeded",

            // Server
            Self::InternalError => "internal_error",
            Self::DatabaseError(_) => "database_error",
            Self::BlockchainError(_) => "blockchain_error",
            Self::ServiceUnavailable => "service_unavailable",
            Self::External(_) => "external_error",
        }
    }

    /// Convert to ErrorResponse
    pub fn to_error_response(&self) -> ErrorResponse {
        let mut response = ErrorResponse::new(self.error_code(), self.to_string());

        // Add details for certain error types
        match self {
            Self::InsufficientPermissions(perm) => {
                response = response.with_details(format!("Required permission: {}", perm));
            }
            Self::DatabaseError(details) | Self::BlockchainError(details) => {
                response = response.with_details(details.clone());
            }
            _ => {}
        }

        response
    }
}

/// Implement IntoResponse for AppError (Axum integration)
///
/// This allows returning AppError directly from Axum handlers.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_response = self.to_error_response();

        // Log error (for monitoring)
        match status {
            StatusCode::INTERNAL_SERVER_ERROR | StatusCode::SERVICE_UNAVAILABLE => {
                tracing::error!(
                    error = ?self,
                    status = ?status,
                    request_id = %error_response.error.request_id,
                    "Server error occurred"
                );
            }
            _ => {
                tracing::warn!(
                    error = ?self,
                    status = ?status,
                    request_id = %error_response.error.request_id,
                    "Request error occurred"
                );
            }
        }

        (status, Json(error_response)).into_response()
    }
}

/// Convenience type alias for Result with AppError
pub type Result<T> = std::result::Result<T, AppError>;

/// Convert SQLx errors to AppError
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::ResourceNotFound("Database record not found".into()),
            sqlx::Error::Database(db_err) => {
                // Check for specific database errors
                if let Some(constraint) = db_err.constraint() {
                    if constraint.contains("unique") {
                        return Self::DuplicateResource(constraint.to_string());
                    }
                    return Self::ConstraintViolation(constraint.to_string());
                }
                Self::DatabaseError(db_err.to_string())
            }
            _ => Self::DatabaseError(err.to_string()),
        }
    }
}

/// Convert validation errors to AppError
impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let message = errors
            .field_errors()
            .iter()
            .map(|(field, errs)| {
                let err_msgs: Vec<String> = errs
                    .iter()
                    .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                    .collect();
                format!("{}: {}", field, err_msgs.join(", "))
            })
            .collect::<Vec<_>>()
            .join("; ");

        Self::ValidationError(message)
    }
}

/// Convert anyhow errors to AppError
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::External(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_creation() {
        let response = ErrorResponse::new("test_error", "Test error message")
            .with_details("Additional details")
            .with_field("test_field");

        assert_eq!(response.error.code, "test_error");
        assert_eq!(response.error.message, "Test error message");
        assert_eq!(response.error.details, Some("Additional details".to_string()));
        assert_eq!(response.error.field, Some("test_field".to_string()));
    }

    #[test]
    fn test_app_error_status_codes() {
        assert_eq!(AppError::MissingToken.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            AppError::InsufficientPermissions("test".into()).status_code(),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            AppError::ValidationError("test".into()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::ResourceNotFound("test".into()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            AppError::DuplicateResource("test".into()).status_code(),
            StatusCode::CONFLICT
        );
        assert_eq!(AppError::AlreadyVoted.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(AppError::RateLimitExceeded.status_code(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(AppError::InternalError.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_codes() {
        assert_eq!(AppError::MissingToken.error_code(), "missing_token");
        assert_eq!(
            AppError::InsufficientPermissions("test".into()).error_code(),
            "insufficient_permissions"
        );
        assert_eq!(AppError::AlreadyVoted.error_code(), "already_voted");
    }
}

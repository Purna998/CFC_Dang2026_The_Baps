use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication failed")]
    Unauthorized,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Token error: {0}")]
    Token(#[from] jsonwebtoken::errors::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Already voted in this election")]
    AlreadyVoted,

    #[error("Election not open")]
    ElectionNotOpen,

    #[error("Voter not verified")]
    VoterNotVerified,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Validation error: {0}")]
    Validation(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Token(_) => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            AppError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Forbidden(ref msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::AlreadyVoted => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::ElectionNotOpen => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::VoterNotVerified => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::Validation(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

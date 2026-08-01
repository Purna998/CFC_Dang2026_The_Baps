//! Health check endpoint

use axum::{extract::State, Json};
use eemp_error::Result;
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    database: String,
}

/// Health check handler
pub async fn health_check(State(state): State<AppState>) -> Result<Json<HealthResponse>> {
    // Check database connection
    let db_status = match state.db.health_check().await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        database: db_status.to_string(),
    }))
}

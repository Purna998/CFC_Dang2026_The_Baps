//! Result route handlers

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use eemp_domain::ElectionId;
use eemp_error::Result;
use eemp_result_service::{dto::*, ResultService};
use std::str::FromStr;

use crate::state::AppState;

/// Create result routes
pub fn result_routes() -> Router<AppState> {
    Router::new()
        .route("/:election_id/calculate", post(calculate_results))
        .route("/:election_id/publish", post(publish_results))
        .route("/:election_id", get(get_results))
}

/// Calculate results handler
///
/// POST /api/v1/results/:election_id/calculate
async fn calculate_results(
    State(state): State<AppState>,
    Path(election_id): Path<String>,
) -> Result<Json<()>> {
    let election_id = ElectionId::from_str(&election_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid election ID: {}", e)))?;

    let result_service = ResultService::new(state.db.clone());
    result_service.calculate_results(election_id).await?;
    Ok(Json(()))
}

/// Publish results handler
///
/// POST /api/v1/results/:election_id/publish
async fn publish_results(
    State(state): State<AppState>,
    Path(election_id): Path<String>,
) -> Result<Json<()>> {
    let election_id = ElectionId::from_str(&election_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid election ID: {}", e)))?;

    let result_service = ResultService::new(state.db.clone());
    result_service.publish_results(election_id).await?;
    Ok(Json(()))
}

/// Get results handler
///
/// GET /api/v1/results/:election_id
async fn get_results(
    State(state): State<AppState>,
    Path(election_id): Path<String>,
) -> Result<Json<ElectionResultsResponse>> {
    let election_id = ElectionId::from_str(&election_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid election ID: {}", e)))?;

    let result_service = ResultService::new(state.db.clone());
    let response = result_service.get_results(election_id).await?;
    Ok(Json(response))
}

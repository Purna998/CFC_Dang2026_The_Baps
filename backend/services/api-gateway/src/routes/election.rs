//! Election route handlers

use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put},
    Json, Router,
};
use eemp_domain::{ElectionId, ElectionStatus};
use eemp_election_service::{dto::*, ElectionService};
use eemp_error::Result;
use serde::Deserialize;
use std::str::FromStr;

use crate::{extractors::ValidatedJson, state::AppState, extractors::AuthUser};

/// Create election routes
pub fn election_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_election))
        .route("/", get(list_elections))
        .route("/:election_id", get(get_election))
        .route("/:election_id/transition", post(transition_election))
        .route("/:election_id/positions", post(create_position))
        .route("/:election_id/positions", get(list_positions))
}

#[derive(Deserialize)]
struct PaginationQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 {
    20
}

/// Create election handler
///
/// POST /api/v1/elections
async fn create_election(
    State(state): State<AppState>,
    auth_user: AuthUser,
    ValidatedJson(req): ValidatedJson<CreateElectionRequest>,
) -> Result<Json<CreateElectionResponse>> {
    let election_service = ElectionService::new(state.db.clone());
    let response = election_service
        .create_election(auth_user.tenant_id, auth_user.user_id, req)
        .await?;
    Ok(Json(response))
}

/// Get election handler
///
/// GET /api/v1/elections/:election_id
async fn get_election(
    State(state): State<AppState>,
    Path(election_id): Path<String>,
) -> Result<Json<ElectionResponse>> {
    let election_id = ElectionId::from_str(&election_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid election ID: {}", e)))?;

    let election_service = ElectionService::new(state.db.clone());
    let response = election_service.get_election(election_id).await?;
    Ok(Json(response))
}

/// List elections handler
///
/// GET /api/v1/elections?limit=20&offset=0
async fn list_elections(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<PaginationQuery>,
) -> Result<Json<ElectionListResponse>> {
    let election_service = ElectionService::new(state.db.clone());
    let response = election_service
        .list_elections(auth_user.tenant_id, params.limit, params.offset)
        .await?;
    Ok(Json(response))
}

/// Transition election status handler
///
/// POST /api/v1/elections/:election_id/transition
async fn transition_election(
    State(state): State<AppState>,
    Path(election_id): Path<String>,
    Json(req): Json<TransitionElectionRequest>,
) -> Result<Json<ElectionResponse>> {
    let election_id = ElectionId::from_str(&election_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid election ID: {}", e)))?;

    let election_service = ElectionService::new(state.db.clone());
    let response = election_service
        .transition_status(election_id, req.new_status)
        .await?;
    Ok(Json(response))
}

/// Create position handler
///
/// POST /api/v1/elections/:election_id/positions
async fn create_position(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(election_id): Path<String>,
    ValidatedJson(req): ValidatedJson<CreatePositionRequest>,
) -> Result<Json<PositionResponse>> {
    let election_id = ElectionId::from_str(&election_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid election ID: {}", e)))?;

    let election_service = ElectionService::new(state.db.clone());
    let response = election_service
        .create_position(auth_user.tenant_id, election_id, req)
        .await?;
    Ok(Json(response))
}

/// List positions handler
///
/// GET /api/v1/elections/:election_id/positions
async fn list_positions(
    State(state): State<AppState>,
    Path(election_id): Path<String>,
) -> Result<Json<Vec<PositionResponse>>> {
    let election_id = ElectionId::from_str(&election_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid election ID: {}", e)))?;

    let election_service = ElectionService::new(state.db.clone());
    let response = election_service.list_positions(election_id).await?;
    Ok(Json(response))
}

//! Voting route handlers

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use eemp_domain::ElectionId;
use eemp_error::Result;
use eemp_voting_service::{dto::*, VotingService};
use std::str::FromStr;

use crate::{
    extractors::{AuthUser, ClientIp, UserAgent, ValidatedJson},
    state::AppState,
};

/// Create voting routes
pub fn voting_routes() -> Router<AppState> {
    Router::new()
        .route("/cast", post(cast_ballot))
        .route("/verify-receipt", post(verify_receipt))
        .route("/status/:election_id", get(get_voting_status))
}

/// Cast ballot handler
///
/// POST /api/v1/voting/cast
async fn cast_ballot(
    State(state): State<AppState>,
    auth_user: AuthUser,
    ClientIp(ip): ClientIp,
    UserAgent(user_agent): UserAgent,
    ValidatedJson(req): ValidatedJson<CastBallotRequest>,
) -> Result<Json<CastBallotResponse>> {
    let voting_service = VotingService::new(state.db.clone());
    let response = voting_service
        .cast_ballot(auth_user.tenant_id, auth_user.user_id, req, ip, user_agent)
        .await?;
    Ok(Json(response))
}

/// Verify receipt handler
///
/// POST /api/v1/voting/verify-receipt
async fn verify_receipt(
    State(state): State<AppState>,
    Json(req): Json<VerifyReceiptRequest>,
) -> Result<Json<VerifyReceiptResponse>> {
    let voting_service = VotingService::new(state.db.clone());
    let response = voting_service.verify_receipt(&req.receipt_code).await?;
    Ok(Json(response))
}

/// Get voting status handler
///
/// GET /api/v1/voting/status/:election_id
async fn get_voting_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(election_id): Path<String>,
) -> Result<Json<VotingStatusResponse>> {
    let election_id = ElectionId::from_str(&election_id)
        .map_err(|e| eemp_error::AppError::ValidationError(format!("Invalid election ID: {}", e)))?;

    let voting_service = VotingService::new(state.db.clone());
    let response = voting_service
        .get_voting_status(election_id, auth_user.user_id)
        .await?;
    Ok(Json(response))
}

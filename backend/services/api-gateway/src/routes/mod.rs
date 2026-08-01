//! API route handlers

pub mod auth;
pub mod health;
pub mod organization;
pub mod election;
pub mod voting;
pub mod result;

use axum::{routing::get, Router};

use crate::state::AppState;

/// Create the main router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health::health_check))
        // Authentication routes
        .nest("/api/v1/auth", auth::auth_routes())
        // Organization routes
        .nest("/api/v1/organizations", organization::organization_routes())
        // Election routes
        .nest("/api/v1/elections", election::election_routes())
        // Voting routes
        .nest("/api/v1/voting", voting::voting_routes())
        // Result routes
        .nest("/api/v1/results", result::result_routes())
        // Share state with all routes
        .with_state(state)
}

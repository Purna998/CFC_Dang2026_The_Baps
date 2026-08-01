//! API Gateway - Main entry point

use axum::Router;
use eemp_config::Config;
use eemp_observability::init_tracing;
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

mod extractors;
mod middleware;
mod routes;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::from_env()?;

    // Initialize tracing
    init_tracing(&config);

    tracing::info!(
        environment = ?config.server.environment,
        "Starting API Gateway"
    );

    // Create application state
    let state = AppState::new(config.clone()).await?;

    // Create router
    let app = create_app(state);

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Listening on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Create the application with all middleware
fn create_app(state: AppState) -> Router {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create router with all routes
    routes::create_router(state)
        // Add middleware layers (applied in reverse order)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        // This is a placeholder test
        // Real tests would use a test database
    }
}

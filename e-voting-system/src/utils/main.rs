mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;
mod routes;

use axum::{
    middleware as axum_middleware,
    routing::{get, post, patch, delete},
    Router, Json,
};
use config::Config;
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Application state
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "e_voting_system=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;

    // Create database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await?;

    let app_state = AppState {
        db: db_pool,
        config: Arc::new(config),
    };

    let addr = format!("{}:{}",
        app_state.config.server_host,
        app_state.config.server_port
    );

    // Health check handler
    async fn health_check() -> Json<serde_json::Value> {
        Json(json!({
            "status": "healthy",
            "service": "e-voting-system",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/api/auth/register", post(handlers::admin::register_voter))
        .route("/api/auth/login", post(handlers::auth::login));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        // User routes
        .route("/api/users/profile", get(handlers::users::get_profile))
        .route("/api/users/password", post(handlers::users::change_password))

        // Admin routes
        .route("/api/admin/voters", get(handlers::admin::list_voters))
        .route("/api/admin/voters/:id/verify", post(handlers::admin::verify_voter))

        // Election routes
        .route("/api/elections", get(handlers::elections::list_elections))
        .route("/api/elections", post(handlers::elections::create_election))
        .route("/api/elections/:id", get(handlers::elections::get_election))
        .route("/api/elections/:id", post(handlers::elections::update_election))
        .route("/api/elections/:id/open", post(handlers::elections::open_election))
        .route("/api/elections/:id/close", post(handlers::elections::close_election))
        .route("/api/elections/:id/archive", post(handlers::elections::archive_election))

        // Party routes
        .route("/api/parties", get(handlers::parties::list_parties))
        .route("/api/parties", post(handlers::parties::create_party))
        .route("/api/parties/:id", get(handlers::parties::get_party))
        .route("/api/parties/:id", patch(handlers::parties::update_party))
        .route("/api/parties/:id", delete(handlers::parties::delete_party))
        .route("/api/admin/parties/:id/verify", post(handlers::parties::verify_party))

        // Candidate routes
        .route("/api/elections/:election_id/candidates", get(handlers::candidates::list_candidates))
        .route("/api/elections/:election_id/candidates", post(handlers::candidates::create_candidate))

        // Voting routes
        .route("/api/elections/:election_id/vote", post(handlers::voting::cast_vote))
        .route("/api/elections/:election_id/verify", post(handlers::voting::verify_vote))

        // Results routes
        .route("/api/elections/:election_id/results", get(handlers::results::get_results))
        .route("/api/elections/:election_id/results/publish", post(handlers::results::publish_results))

        // Apply authentication middleware to all protected routes
        .layer(axum_middleware::from_fn_with_state(
            app_state.clone(),
            middleware::auth::auth_middleware
        ));

    // Combine routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    tracing::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

//! Application state shared across all handlers

use eemp_auth_service::AuthService;
use eemp_config::Config;
use eemp_database::Database;
use eemp_error::Result;
use std::sync::Arc;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub auth_service: Arc<AuthService>,
    pub config: Arc<Config>,
}

impl AppState {
    /// Create a new application state
    pub async fn new(config: Config) -> Result<Self> {
        let db = Database::new(&config.database).await?;
        let auth_service = Arc::new(AuthService::new(db.clone(), config.clone())?);

        Ok(Self {
            db,
            auth_service,
            config: Arc::new(config),
        })
    }
}

//! Observability setup for EEMP
//!
//! Provides structured logging with tracing and OpenTelemetry.

use eemp_config::{Config, Environment};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize tracing for the application
pub fn init_tracing(config: &Config) {
    let env_filter = match config.server.environment {
        Environment::Development => {
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))
        }
        Environment::Staging => {
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
        }
        Environment::Production => {
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
        }
    };

    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .json(); // JSON format for structured logging

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .init();

    tracing::info!(
        environment = ?config.server.environment,
        "Tracing initialized"
    );
}

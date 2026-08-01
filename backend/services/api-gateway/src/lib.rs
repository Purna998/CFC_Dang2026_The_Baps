//! API Gateway for EEMP
//!
//! Provides:
//! - HTTP API endpoints
//! - Request routing
//! - Middleware (auth, tenant resolution, rate limiting)
//! - CORS configuration

pub mod middleware;
pub mod routes;
pub mod state;
pub mod extractors;

pub use state::AppState;

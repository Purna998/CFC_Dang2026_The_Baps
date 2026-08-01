//! Rate limiting middleware

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use eemp_error::{AppError, ErrorResponse};
use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::{num::NonZeroU32, sync::Arc};

/// Rate limiter using governor
pub struct AppRateLimiter {
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl AppRateLimiter {
    /// Create a new rate limiter
    ///
    /// Default: 100 requests per minute
    pub fn new(requests_per_minute: u32) -> Self {
        let quota = Quota::per_minute(NonZeroU32::new(requests_per_minute).unwrap());
        let limiter = Arc::new(RateLimiter::direct(quota));

        Self { limiter }
    }

    /// Check if a request is allowed
    pub fn check(&self) -> bool {
        self.limiter.check().is_ok()
    }
}

impl Default for AppRateLimiter {
    fn default() -> Self {
        Self::new(100)
    }
}

/// Rate limiting middleware
pub async fn rate_limit_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // In a real implementation, this would:
    // 1. Extract client identifier (IP address, API key, user ID)
    // 2. Check rate limit per identifier
    // 3. Return 429 if exceeded
    // 4. Add rate limit headers (X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset)

    // For now, just pass through
    // TODO: Implement per-client rate limiting with Redis

    Ok(next.run(request).await)
}

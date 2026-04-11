//! HTTP API routes for convergio-scheduler.

use axum::Router;

/// Returns the router for this crate's API endpoints.
pub fn routes() -> Router {
    Router::new()
    // .route("/api/scheduler/health", get(health))
}

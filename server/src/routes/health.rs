/// Defines the health check route for the server using Axum.
///
/// This module imports necessary components from the Axum framework,
/// specifically the `get` routing method and the `Router` type,
/// to set up HTTP GET endpoints for health checks.
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/", get(health_check))
}

async fn health_check() -> &'static str {
    "✅ TagBlaze is healthy!"
}

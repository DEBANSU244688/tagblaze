use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(health_check))
}

async fn health_check() -> &'static str {
    "✅ TagBlaze is healthy!"
}
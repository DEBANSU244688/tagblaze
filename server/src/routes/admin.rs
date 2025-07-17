use axum::{routing::post, Router};
use crate::handlers::admin::reset_db;

pub fn routes() -> Router {
    Router::new().route("/reset-db", post(reset_db))
}
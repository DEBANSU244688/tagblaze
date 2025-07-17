/// Resets the database to its initial state.
///
/// This function is typically used for administrative purposes to clear all data
/// and restore the database to its default configuration. Use with caution, as
/// all existing data will be lost.
///
/// # Errors
/// Returns an error if the database reset operation fails.
///
/// # Example
/// ```
/// reset_db();
/// ```
use crate::handlers::admin::reset_db;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new().route("/reset-db", post(reset_db))
}

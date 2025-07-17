use crate::handlers::{
    auth::{login_user, me, register_user},
    ticket::get_tickets,
};
use axum::{
    Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize)]
/// Represents a request to register a new user.
///
/// # Fields
/// - `email`: The email address of the user.
/// - `name`: The display name of the user.
/// - `password`: The password for the user's account.
/// - `role`: The role assigned to the user, either `"agent"` or `"admin"`.
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub password: String,
    pub role: String, // "agent" or "admin"
}

pub fn routes() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/me", get(me))
        .route("/", get(get_tickets))
}

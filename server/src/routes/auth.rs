use axum::{routing::{post}, Router};
use crate::handlers::auth::{register_user, login_user};
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
}

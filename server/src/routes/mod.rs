pub mod health;
pub mod auth;
pub mod ticket;
pub mod user;
pub mod tag;
pub mod relations;
pub mod admin;

use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .nest("/health", health::routes())
        .nest("/auth", auth::routes())
        .nest("/tickets", ticket::routes())
        .nest("/users", user::routes())
        .nest("/tags", tag::routes())
        .nest("/relations", relations::routes()) 
        .nest("/admin/dev", admin::routes())
}
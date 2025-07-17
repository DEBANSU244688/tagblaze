pub mod admin;
pub mod auth;
pub mod health;
pub mod relations;
pub mod tag;
pub mod ticket;

/// Imports the `Router` type from the `axum` crate, which is used to define and compose HTTP routes and middleware
/// for building web applications and APIs in Rust.
///
/// The `Router` provides methods to add routes, nest routers, and apply middleware, enabling modular and scalable
/// server architectures.
///
/// # Example
/// ```rust
/// use axum::Router;
///
/// let app = Router::new();
/// ```
use axum::Router;

pub fn create_router() -> Router {
    Router::new()
        .nest("/health", health::routes())
        .nest("/auth", auth::routes())
        .nest("/tickets", ticket::routes())
        .nest("/tags", tag::routes())
        .nest("/relations", relations::routes())
        .nest("/admin/dev", admin::routes())
}

use std::net::SocketAddr;
use tracing_subscriber;

/// Handles application-wide configuration settings
mod config;

/// Manages database connections, migrations, and queries
mod db;

/// Contains all HTTP request handlers
mod handlers;

/// Defines shared data models used across the application
mod models;

/// Configures all application routes and middleware
mod routes;

/// Utility functions and helpers used across multiple modules
mod utils;

/// Entry point for the TagBlaze application.
/// 
/// This function sets up logging, configures the application router,
/// binds the server to a local address, and starts the Axum HTTP server.
#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for structured logging
    tracing_subscriber::fmt::init();

    // Construct the full application router from all defined routes
    let app = routes::create_router();

    // Define the local address for the server to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 TagBlaze running at http://{}", addr);

    // Start the Axum server with the configured router
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

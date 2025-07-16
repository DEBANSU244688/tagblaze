use std::net::SocketAddr;
use tracing_subscriber;

mod routes;
mod handlers;
mod db;
mod models;
mod config;
mod utils;

#[tokio::main]
async fn main() {
    // Initialize logs
    tracing_subscriber::fmt::init();

    // Build the app router with all routes pre-merged
    let app = routes::create_router();

    // Define the address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 TagBlaze running at http://{}", addr);

    // Start the Axum server
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
use std::net::SocketAddr;
use tracing_subscriber;

mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    // Initialize logs
    tracing_subscriber::fmt::init();

    // Build the app router with all routes pre-merged
    let app = routes::create_router();

    // Define the address
    /// The `addr` variable specifies the socket address for the server to bind to.
    /// It uses the IPv4 loopback address (`127.0.0.1`) and port `3000`, meaning the server
    /// will only be accessible locally on the specified port.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 TagBlaze running at http://{}", addr);

    // Start the Axum server
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

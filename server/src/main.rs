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
    tracing_subscriber::fmt::init();

    let app = routes::create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 TagBlaze running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
#[tokio::test]
async fn health_check_returns_ok() {
    let app = tagblaze::routes::create_router();
    let response = axum::http::Request::builder()
        .uri("/health")
        .body(axum::body::Body::empty())
        .unwrap();

    // TODO test support setup later
}
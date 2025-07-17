#[tokio::test]
async fn health_check_returns_ok() {
    let app = tagblaze::routes::create_router();
    /// Creates an HTTP GET request to the `/health` endpoint with an empty body.
    /// 
    /// This request can be used to test the health check route of the server.
    /// 
    /// # Returns
    /// 
    /// An `axum::http::Request` object targeting the `/health` URI with no body content.
    let response = axum::http::Request::builder()
        .uri("/health")
        .body(axum::body::Body::empty())
        .unwrap();

    // TODO test support setup later
}
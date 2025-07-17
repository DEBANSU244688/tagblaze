use crate::utils::jwt::Claims;
use axum::http::Request;
use axum::http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};

/// Extracts and decodes JWT claims from an incoming HTTP request's `Authorization` header.
///
/// # Parameters
/// - `req`: The HTTP request from which the token is extracted.
///
/// # Returns
/// - `Ok(Claims)` if the token is present, valid, and successfully decoded.
/// - `Err(StatusCode::UNAUTHORIZED)` if the header is missing, malformed, or the token is invalid.
///
/// # Panics
/// - If the environment variable `JWT_SECRET` is not set.
///
/// # Usage
/// This function is typically used in custom extractors or middleware to
/// authorize access to protected routes.
///
/// ```rust
/// let claims = extract_claims(&request)?;
/// println!("User ID from token: {}", claims.sub);
/// ```
pub fn extract_claims<B>(req: &Request<B>) -> Result<Claims, StatusCode> {
    // 🔐 Step 1: Extract "Authorization" header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 🪪 Step 2: Validate the "Bearer <token>" format
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 🧬 Step 3: Retrieve the JWT secret from environment
    let secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set"); // This panic is intentional for critical misconfiguration

    // 🔓 Step 4: Decode the token and validate its signature & structure
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?; // If decoding fails, return 401

    // ✅ Step 5: Return the validated claims
    Ok(token_data.claims)
}
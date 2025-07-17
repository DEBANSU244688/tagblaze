use crate::utils::jwt::Claims;
use axum::http::Request;
use axum::http::StatusCode;
use jsonwebtoken::{DecodingKey, Validation, decode};

pub fn extract_claims<B>(req: &Request<B>) -> Result<Claims, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    /// Attempts to decode a JWT token into `Claims` using the provided secret key and default validation.
    ///
    /// # Arguments
    /// * `token` - The JWT token string to decode.
    /// * `secret` - The secret key used for decoding the token.
    ///
    /// # Errors
    /// Returns a `StatusCode::UNAUTHORIZED` error if the token cannot be decoded or is invalid.
    ///
    /// # Returns
    /// On success, returns the decoded token data as `token_data`.
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(token_data.claims)
}

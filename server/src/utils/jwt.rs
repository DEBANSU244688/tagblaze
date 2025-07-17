use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// The payload structure embedded within a JWT token.
/// 
/// - `sub`: Subject identifier (usually a unique user ID or email).
/// - `exp`: Expiration time as a UNIX timestamp (in seconds).
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Typically the user's email or ID
    pub exp: usize,  // Expiration timestamp (as seconds since epoch)
}

/// Creates a JWT token for a given subject (e.g., user ID or email).
///
/// # Arguments
/// - `sub`: The subject (typically the user ID or email).
/// - `secret`: Secret key used to sign the token.
///
/// # Returns
/// - `Ok(String)`: The generated JWT string.
/// - `Err(jsonwebtoken::errors::Error)`: If token creation fails.
///
/// # Example
/// ```rust
/// let token = create_jwt("user@example.com", "my-secret-key")?;
/// ```
pub fn create_jwt(sub: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // Set token expiration to 24 hours from now
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: sub.to_owned(),
        exp: expiration as usize,
    };

    // Sign the JWT with the provided secret
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Decodes and validates a JWT string, extracting the embedded claims.
///
/// # Arguments
/// - `token`: The JWT string to decode and verify.
///
/// # Returns
/// - `Ok(Claims)`: The decoded claims if the token is valid.
/// - `Err(StatusCode::UNAUTHORIZED)`: If the token is invalid or expired.
///
/// # Panics
/// - If the `JWT_SECRET` environment variable is not set.
///
/// # Example
/// ```rust
/// let claims = extract_claims(token)?;
/// println!("Token subject: {}", claims.sub);
/// ```
pub fn extract_claims(token: &str) -> Result<Claims, StatusCode> {
    // Fetch secret key from environment
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Decode and validate the token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?; // Any failure results in 401

    Ok(token_data.claims)
}
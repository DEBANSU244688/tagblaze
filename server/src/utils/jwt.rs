use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user ID or email
    pub exp: usize,  // expiration timestamp
}

pub fn create_jwt(sub: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: sub.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn extract_claims(token: &str) -> Result<Claims, StatusCode> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    /// Attempts to decode a JWT token into the specified `Claims` type using the provided secret key.
    ///
    /// # Arguments
    /// * `token` - The JWT token string to be decoded.
    /// * `secret` - The secret key used for decoding the token.
    ///
    /// # Returns
    /// Returns `token_data` containing the decoded claims if successful.
    ///
    /// # Errors
    /// Returns a `StatusCode::UNAUTHORIZED` error if the token cannot be decoded or is invalid.
    ///
    /// # Example
    /// ```rust
    /// let token_data = decode::<Claims>(
    ///     token,
    ///     &DecodingKey::from_secret(secret.as_bytes()),
    ///     &Validation::default(),
    /// )
    /// .map_err(|_| StatusCode::UNAUTHORIZED)?;
    /// ```
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(token_data.claims)
}

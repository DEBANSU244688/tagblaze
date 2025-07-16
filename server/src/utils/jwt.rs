use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use axum::http::StatusCode;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user ID or email
    pub exp: usize,   // expiration timestamp
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

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn extract_claims(token: &str) -> Result<Claims, StatusCode> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(token_data.claims)
}
use axum::{
    extract::{Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{EntityTrait, Set, ActiveModelTrait, ColumnTrait, QueryFilter};
use serde::Deserialize;
use crate::{
    models::{ticket, user},
    db::db::connect,
    utils::jwt::extract_claims,
};

#[derive(Deserialize)]
pub struct CreateTicket {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
}

pub async fn create_ticket(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<CreateTicket>,
) -> impl IntoResponse {
    // Extract and validate the token
    let claims = match extract_claims(bearer.token()) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // Connect to DB
    let db = connect().await;

    // Find user by email (claims.sub)
    let user_record = match user::Entity::find()
        .filter(user::Column::Email.eq(claims.sub.clone()))
        .one(&db)
        .await
        .unwrap()
    {
        Some(u) => u,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // Prepare ActiveModel
    let new_ticket = ticket::ActiveModel {
        title: Set(payload.title),
        description: Set(payload.description),
        status: Set(Some(payload.status.unwrap_or("open".into()))),
        user_id: Set(Some(user_record.id)),
        ..Default::default()
    };

    // Insert and respond
    match new_ticket.insert(&db).await {
        Ok(saved_ticket) => axum::Json(saved_ticket).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub fn routes() -> Router {
    Router::new().route("/create", post(create_ticket))
}
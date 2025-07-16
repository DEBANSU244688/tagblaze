use axum::{
    extract::Json,
    response::IntoResponse,
    http::StatusCode,
    Router,
    routing::post,
};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{ActiveModelTrait, Set};
use crate::{
    db::db::connect,
    utils::jwt::extract_claims,
    models::ticket,
};
use serde::Deserialize;

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
    let token = bearer.token(); 

    let claims = match extract_claims(token) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let ticket = ticket::ActiveModel {
        title: Set(payload.title),
        description: Set(payload.description),
        status: Set(Some(payload.status.unwrap_or("open".into()))),
        user_id: Set(Some(claims.sub.parse::<i32>().unwrap())),
        ..Default::default()
    };

    let db = connect().await;
    match ticket.insert(&db).await {
        Ok(saved_ticket) => axum::Json(saved_ticket).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub fn routes() -> Router {
    Router::new().route("/create", post(create_ticket))
}
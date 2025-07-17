use axum::{
    extract::{Json, Path}, response::IntoResponse, http::StatusCode,
};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{
    EntityTrait, Set, ActiveModelTrait, ColumnTrait, QueryFilter, IntoActiveModel
};
use serde::Deserialize;
use chrono::Local;

use crate::{
    models::{ticket, user}, db::db::connect, utils::jwt::extract_claims,
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
    let claims = match extract_claims(bearer.token()) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let db = connect().await;

    let user_record = match user::Entity::find()
        .filter(user::Column::Email.eq(claims.sub.clone()))
        .one(&db)
        .await
        .unwrap()
    {
        Some(u) => u,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let now = Local::now().naive_local();

    let new_ticket = ticket::ActiveModel {
        title: Set(payload.title),
        description: Set(payload.description),
        status: Set(Some(payload.status.unwrap_or("open".into()))),
        user_id: Set(Some(user_record.id)),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };

    match new_ticket.insert(&db).await {
        Ok(saved_ticket) => axum::Json(saved_ticket).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_tickets(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let db = connect().await;

    let claims = match extract_claims(bearer.token()) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let user = match user::Entity::find()
        .filter(user::Column::Email.eq(claims.sub.clone()))
        .one(&db)
        .await
        .unwrap()
    {
        Some(u) => u,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let tickets = if user.role == "admin" {
        ticket::Entity::find().all(&db).await
    } else {
        ticket::Entity::find()
            .filter(ticket::Column::UserId.eq(Some(user.id)))
            .all(&db)
            .await
    };

    match tickets {
        Ok(list) => Json(list).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_ticket_by_id(
    Path(ticket_id): Path<i32>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let db = connect().await;

    let claims = match extract_claims(bearer.token()) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let ticket = match ticket::Entity::find_by_id(ticket_id)
        .one(&db)
        .await
        .unwrap()
    {
        Some(t) => t,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let user = match user::Entity::find()
        .filter(user::Column::Email.eq(claims.sub.clone()))
        .one(&db)
        .await
        .unwrap()
    {
        Some(u) => u,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    if user.role != "admin" && Some(user.id) != ticket.user_id {
        return StatusCode::FORBIDDEN.into_response();
    }

    Json(ticket).into_response()
}

pub async fn delete_ticket_by_id(
    Path(ticket_id): Path<i32>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let db = connect().await;

    let claims = match extract_claims(bearer.token()) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let user = match user::Entity::find()
        .filter(user::Column::Email.eq(claims.sub.clone()))
        .one(&db)
        .await
        .unwrap()
    {
        Some(u) => u,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let ticket = match ticket::Entity::find_by_id(ticket_id)
        .one(&db)
        .await
        .unwrap()
    {
        Some(t) => t,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    if user.role != "admin" && ticket.user_id != Some(user.id) {
        return StatusCode::FORBIDDEN.into_response();
    }

    match ticket.into_active_model().delete(&db).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Deserialize)]
pub struct UpdateTicket {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

pub async fn update_ticket_by_id(
    Path(ticket_id): Path<i32>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<UpdateTicket>,
) -> impl IntoResponse {
    let db = connect().await;

    let claims = match extract_claims(bearer.token()) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let user = match user::Entity::find()
        .filter(user::Column::Email.eq(claims.sub.clone()))
        .one(&db)
        .await
        .unwrap()
    {
        Some(u) => u,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let ticket = match ticket::Entity::find_by_id(ticket_id)
        .one(&db)
        .await
        .unwrap()
    {
        Some(t) => t,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    if user.role != "admin" && ticket.user_id != Some(user.id) {
        return StatusCode::FORBIDDEN.into_response();
    }

    let mut active_ticket: ticket::ActiveModel = ticket.into_active_model();
    if let Some(t) = payload.title {
        active_ticket.title = Set(t);
    }
    if let Some(d) = payload.description {
        active_ticket.description = Set(Some(d));
    }
    if let Some(s) = payload.status {
        active_ticket.status = Set(Some(s));
    }

    active_ticket.updated_at = Set(Some(Local::now().naive_local()));

    match active_ticket.update(&db).await {
        Ok(updated) => axum::Json(updated).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
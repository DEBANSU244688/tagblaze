use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::TypedHeader;
use chrono::Local;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::Deserialize;

use crate::{
    db::db::connect,
    models::{ticket, user},
    utils::jwt::extract_claims,
};

/// Payload for creating a new ticket.
#[derive(Deserialize)]
pub struct CreateTicket {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// Create a new ticket assigned to the authenticated user.
///
/// # Headers
/// - `Authorization: Bearer <token>`
///
/// # Request Body
/// - `title`: Title of the ticket (required)
/// - `description`: Optional description
/// - `status`: Optional status (defaults to `"open"`)
///
/// # Returns
/// - `200 OK` with the created ticket
/// - `401 UNAUTHORIZED` if JWT is invalid
/// - `500 INTERNAL_SERVER_ERROR` on DB failure
pub async fn create_ticket(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<CreateTicket>,
) -> impl IntoResponse {
    let claims = match extract_claims(bearer.token()) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let db = connect().await;

    // 🎯 Get user from JWT claim
    let user_record = match user::Entity::find()
        .filter(user::Column::Email.eq(claims.sub.clone()))
        .one(&db)
        .await
        .unwrap()
    {
        Some(u) => u,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // 🕒 Timestamp now
    let now = Local::now().naive_local();

    // 📦 Build ticket model
    let new_ticket = ticket::ActiveModel {
        title: Set(payload.title),
        description: Set(payload.description),
        status: Set(Some(payload.status.unwrap_or("open".into()))),
        user_id: Set(Some(user_record.id)),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };

    // 💾 Insert into DB
    match new_ticket.insert(&db).await {
        Ok(saved_ticket) => axum::Json(saved_ticket).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Get all tickets available to the authenticated user.
///
/// - Admins receive **all** tickets.
/// - Regular users receive only **their own** tickets.
///
/// # Returns
/// - `200 OK` with ticket list
/// - `401 UNAUTHORIZED` if JWT is invalid
/// - `500 INTERNAL_SERVER_ERROR` on DB failure
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

    // 🧠 Admins get all tickets, others get only their own
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

/// Get a specific ticket by ID (with access control).
///
/// - Admins can view any ticket.
/// - Regular users can only view their own tickets.
///
/// # Returns
/// - `200 OK` with ticket
/// - `403 FORBIDDEN` if access is denied
/// - `404 NOT_FOUND` if ticket doesn't exist
/// - `401 UNAUTHORIZED` if JWT is invalid
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

    // 🚫 Access control
    if user.role != "admin" && Some(user.id) != ticket.user_id {
        return StatusCode::FORBIDDEN.into_response();
    }

    Json(ticket).into_response()
}

/// Delete a ticket by ID (with access control).
///
/// - Admins can delete any ticket.
/// - Regular users can only delete their own tickets.
///
/// # Returns
/// - `204 NO_CONTENT` on success
/// - `403 FORBIDDEN` if unauthorized
/// - `404 NOT_FOUND` if ticket doesn't exist
/// - `401 UNAUTHORIZED` if JWT is invalid
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

    // 🛡️ Only allow deletion if owner or admin
    if user.role != "admin" && ticket.user_id != Some(user.id) {
        return StatusCode::FORBIDDEN.into_response();
    }

    match ticket.into_active_model().delete(&db).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Payload for updating a ticket.
#[derive(Deserialize)]
pub struct UpdateTicket {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// Update a ticket by ID (with access control).
///
/// # Request Body
/// - Optional fields to update: `title`, `description`, `status`
///
/// # Returns
/// - `200 OK` with updated ticket
/// - `403 FORBIDDEN` if access denied
/// - `404 NOT_FOUND` if ticket doesn't exist
/// - `401 UNAUTHORIZED` if JWT is invalid
/// - `500 INTERNAL_SERVER_ERROR` on update failure
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

    // 🔐 Enforce ownership or admin access
    if user.role != "admin" && ticket.user_id != Some(user.id) {
        return StatusCode::FORBIDDEN.into_response();
    }

    // 🛠️ Apply patch
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
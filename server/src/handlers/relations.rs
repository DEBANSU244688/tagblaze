use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;

use crate::{
    db::db::connect,
    models::{tag, ticket_tag, ticket_tag::Entity as TicketTagEntity},
    utils::jwt::extract_claims,
};

/// Attach a tag to a ticket (create a relation).
///
/// Requires a valid JWT bearer token for authentication.
///
/// # Path Params
/// - `ticket_id`: ID of the ticket
/// - `tag_id`: ID of the tag to attach
///
/// # Headers
/// - `Authorization: Bearer <token>`
///
/// # Returns
/// - `201 CREATED` on success
/// - `409 CONFLICT` if the relation already exists
/// - `401 UNAUTHORIZED` if token is invalid
pub async fn attach_tag(
    Path((ticket_id, tag_id)): Path<(i32, i32)>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    // 🛡️ Authenticate the request via JWT
    if extract_claims(bearer.token()).is_err() {
        return StatusCode::UNAUTHORIZED;
    }

    let db = connect().await;

    // 🔗 Create new tag-ticket relation
    let link = ticket_tag::ActiveModel {
        ticket_id: Set(ticket_id),
        tag_id: Set(tag_id),
        ..Default::default()
    };

    // 💾 Try to insert relation into DB
    match link.insert(&db).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}

/// Fetch all tags associated with a given ticket.
///
/// # Path Params
/// - `ticket_id`: ID of the ticket to fetch tags for
///
/// # Returns
/// - `200 OK` with a JSON array of tag objects
/// - `500 INTERNAL_SERVER_ERROR` on failure
pub async fn get_tags_for_ticket(Path(ticket_id): Path<i32>) -> impl IntoResponse {
    let db: DatabaseConnection = connect().await;

    // 🔍 Find all ticket_tag entries related to this ticket, with tag data joined
    match ticket_tag::Entity::find()
        .filter(ticket_tag::Column::TicketId.eq(ticket_id))
        .find_also_related(tag::Entity)
        .all(&db)
        .await
    {
        Ok(pairs) => {
            // Extract only the tag part of the (ticket_tag, tag) pair
            let tags: Vec<_> = pairs
                .into_iter()
                .filter_map(|(_, maybe_tag)| maybe_tag)
                .collect();

            Json(tags).into_response()
        }
        Err(e) => {
            eprintln!("❌ Failed to fetch tags for ticket {}: {:?}", ticket_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to fetch tags",
                    "details": e.to_string(),
                    "ticket_id": ticket_id
                })),
            ).into_response()
        }
    }
}

/// Detach a tag from a ticket (delete the relation).
///
/// # Path Params
/// - `ticket_id`: ID of the ticket
/// - `tag_id`: ID of the tag to detach
///
/// # Returns
/// - `204 NO_CONTENT` on success
/// - `500 INTERNAL_SERVER_ERROR` on failure
pub async fn detach_tag(Path((ticket_id, tag_id)): Path<(i32, i32)>) -> impl IntoResponse {
    let db = connect().await;

    // 🗑️ Delete the specific ticket-tag relation
    match TicketTagEntity::delete_many()
        .filter(ticket_tag::Column::TicketId.eq(ticket_id))
        .filter(ticket_tag::Column::TagId.eq(tag_id))
        .exec(&db)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
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

pub async fn attach_tag(
    Path((ticket_id, tag_id)): Path<(i32, i32)>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    if extract_claims(bearer.token()).is_err() {
        return StatusCode::UNAUTHORIZED;
    }

    let db = connect().await;

    /// Creates a new `ticket_tag::ActiveModel` instance representing the association between a ticket and a tag.
    ///
    /// # Arguments
    ///
    /// * `ticket_id` - The identifier of the ticket to be linked.
    /// * `tag_id` - The identifier of the tag to be linked.
    ///
    /// The remaining fields are set to their default values.
    let link = ticket_tag::ActiveModel {
        ticket_id: Set(ticket_id),
        tag_id: Set(tag_id),
        ..Default::default()
    };

    match link.insert(&db).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::CONFLICT,
    }
}

pub async fn get_tags_for_ticket(Path(ticket_id): Path<i32>) -> impl IntoResponse {
    let db: DatabaseConnection = connect().await;

    match ticket_tag::Entity::find()
        .filter(ticket_tag::Column::TicketId.eq(ticket_id))
        .find_also_related(tag::Entity)
        .all(&db)
        .await
    {
        Ok(pairs) => {
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
            )
                .into_response()
        }
    }
}

pub async fn detach_tag(Path((ticket_id, tag_id)): Path<(i32, i32)>) -> impl IntoResponse {
    let db = connect().await;

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

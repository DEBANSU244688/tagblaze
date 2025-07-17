use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::TypedHeader;
use chrono::Local;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait, Set};
use serde::Deserialize;

use crate::{db::db::connect, models::tag, utils::jwt::extract_claims};

/// Payload for creating a new tag.
#[derive(Deserialize)]
pub struct CreateTag {
    pub name: String,
}

/// Payload for updating an existing tag.
#[derive(Deserialize)]
pub struct UpdateTag {
    pub name: Option<String>,
}

/// Create a new tag.
///
/// Requires a valid bearer token. Accepts a JSON payload with the tag name.
/// Timestamps for `created_at` and `updated_at` are automatically set.
///
/// # Returns
/// - `200 OK` with the created tag
/// - `401 UNAUTHORIZED` if token is missing/invalid
/// - `500 INTERNAL_SERVER_ERROR` on DB failure
pub async fn create_tag(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<CreateTag>,
) -> impl IntoResponse {
    // 🛡️ Validate JWT token
    let _claims = match extract_claims(bearer.token()) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let db = connect().await;
    let now = Local::now().naive_local();

    // 🧱 Construct new tag ActiveModel
    let new_tag = tag::ActiveModel {
        name: Set(payload.name),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };

    // 💾 Insert into DB
    match new_tag.insert(&db).await {
        Ok(saved_tag) => axum::Json::<tag::Model>(saved_tag).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Fetch all tags.
///
/// Public route that returns a list of all tags in the database.
///
/// # Returns
/// - `200 OK` with array of tags
/// - `500 INTERNAL_SERVER_ERROR` on DB failure
pub async fn get_tags() -> impl IntoResponse {
    let db = connect().await;
    match tag::Entity::find().all(&db).await {
        Ok(tags) => axum::Json::<Vec<tag::Model>>(tags).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Fetch a single tag by its ID.
///
/// # Path Parameters
/// - `id`: ID of the tag to retrieve
///
/// # Returns
/// - `200 OK` with tag object
/// - `404 NOT_FOUND` if tag doesn't exist
/// - `500 INTERNAL_SERVER_ERROR` on DB failure
pub async fn get_tag_by_id(Path(id): Path<i32>) -> impl IntoResponse {
    let db = connect().await;
    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(tag)) => axum::Json::<tag::Model>(tag).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Update an existing tag by its ID.
///
/// Accepts a partial update payload. Only the tag name is currently updatable.
///
/// # Path Parameters
/// - `id`: ID of the tag to update
///
/// # JSON Payload
/// - `name` (optional): New name for the tag
///
/// # Returns
/// - `200 OK` with updated tag
/// - `400 BAD_REQUEST` if no updatable fields are provided
/// - `404 NOT_FOUND` if tag doesn't exist
/// - `500 INTERNAL_SERVER_ERROR` on DB failure
pub async fn update_tag_by_id(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTag>,
) -> impl IntoResponse {
    let db = connect().await;

    // 🔍 Fetch the existing tag
    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(existing)) => {
            let mut active = existing.into_active_model();

            // 📝 Apply update if field is provided
            if let Some(new_name) = payload.name.clone() {
                active.name = Set(new_name);
                active.updated_at = Set(Some(Local::now().naive_local()));
            } else {
                return StatusCode::BAD_REQUEST.into_response(); // 🚫 No updates provided
            }

            // 💾 Save updated tag
            match active.update(&db).await {
                Ok(_) => match tag::Entity::find_by_id(id).one(&db).await {
                    Ok(Some(updated_tag)) => Json(updated_tag).into_response(),
                    _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                },
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Delete a tag by its ID.
///
/// # Path Parameters
/// - `id`: ID of the tag to delete
///
/// # Returns
/// - `204 NO_CONTENT` on success
/// - `404 NOT_FOUND` if tag doesn't exist
/// - `500 INTERNAL_SERVER_ERROR` on DB failure
pub async fn delete_tag_by_id(Path(id): Path<i32>) -> impl IntoResponse {
    let db = connect().await;

    // 🔍 Fetch and delete tag if it exists
    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(tag)) => match tag.delete(&db).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Ok(None) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
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

#[derive(Deserialize)]
pub struct CreateTag {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateTag {
    pub name: Option<String>,
}

// ✅ Create Tag
pub async fn create_tag(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<CreateTag>,
) -> impl IntoResponse {
    let _claims = match extract_claims(bearer.token()) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let db = connect().await;
    let now = Local::now().naive_local();

    /// Creates a new `tag::ActiveModel` instance with the provided name and timestamps.
    ///
    /// # Arguments
    /// * `payload.name` - The name of the tag to be created.
    /// * `now` - The current timestamp to set for both `created_at` and `updated_at`.
    ///
    /// # Fields
    /// - `name`: Sets the tag's name from the payload.
    /// - `created_at`: Sets the creation timestamp.
    /// - `updated_at`: Sets the update timestamp.
    /// - Other fields are set to their default values.
    ///
    /// This is typically used when inserting a new tag record into the database.
    let new_tag = tag::ActiveModel {
        name: Set(payload.name),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };

    match new_tag.insert(&db).await {
        Ok(saved_tag) => axum::Json::<tag::Model>(saved_tag).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ✅ Read All Tags
pub async fn get_tags() -> impl IntoResponse {
    let db = connect().await;
    match tag::Entity::find().all(&db).await {
        Ok(tags) => axum::Json::<Vec<tag::Model>>(tags).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ✅ Read Tag by ID
pub async fn get_tag_by_id(Path(id): Path<i32>) -> impl IntoResponse {
    let db = connect().await;
    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(tag)) => axum::Json::<tag::Model>(tag).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ✅ Update Tag
pub async fn update_tag_by_id(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTag>,
) -> impl IntoResponse {
    let db = connect().await;

    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(existing)) => {
            let mut active = existing.into_active_model();

            if let Some(new_name) = payload.name.clone() {
                active.name = Set(new_name);
                active.updated_at = Set(Some(Local::now().naive_local()));
            } else {
                return StatusCode::BAD_REQUEST.into_response();
            }

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

// ✅ Delete Tag
pub async fn delete_tag_by_id(Path(id): Path<i32>) -> impl IntoResponse {
    let db = connect().await;
    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(tag)) => match tag.delete(&db).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Ok(None) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

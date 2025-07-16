use axum::{
    extract::{Json, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{post, get},
    Router,
};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use sea_orm::{EntityTrait, Set, ActiveModelTrait, ModelTrait, IntoActiveModel};
use serde::{Deserialize};
use chrono::{Local, FixedOffset};
use crate::{
    models::tag,
    db::db::connect,
    utils::jwt::extract_claims,
};

#[derive(Deserialize)]
pub struct CreateTag {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateTag {
    pub name: Option<String>,
}

// ✅ Create
pub async fn create_tag(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<CreateTag>,
) -> impl IntoResponse {
    let _claims = match extract_claims(bearer.token()) {
        Ok(c) => c,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let db = connect().await;

    let new_tag = tag::ActiveModel {
        name: Set(payload.name),
        ..Default::default()
    };

    match new_tag.insert(&db).await {
        Ok(saved_tag) => axum::Json::<tag::Model>(saved_tag).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ✅ Read All
pub async fn get_tags() -> impl IntoResponse {
    let db = connect().await;
    match tag::Entity::find().all(&db).await {
        Ok(tags) => axum::Json::<Vec<tag::Model>>(tags).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ✅ Read by ID
pub async fn get_tag_by_id(Path(id): Path<i32>) -> impl IntoResponse {
    let db = connect().await;
    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(tag)) => axum::Json::<tag::Model>(tag).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

// ✅ Update
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
                active.updated_at = Set(Some(Local::now().with_timezone(&FixedOffset::east_opt(0).unwrap())));
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

// ✅ Delete
pub async fn delete_tag_by_id(Path(id): Path<i32>) -> impl IntoResponse {
    let db = connect().await;
    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(tag)) => match tag.delete(&db).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
        Ok(None) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// ✅ Routes
pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_tag).get(get_tags))
        .route("/{id}", get(get_tag_by_id)
                      .put(update_tag_by_id)
                      .delete(delete_tag_by_id))
}
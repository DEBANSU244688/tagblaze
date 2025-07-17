use axum::{
    routing::{post, get}, Router,
};
use crate::handlers::tag::{
    create_tag, get_tags, get_tag_by_id, update_tag_by_id, delete_tag_by_id
};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_tag).get(get_tags))
        .route("/{id}", get(get_tag_by_id)
                      .put(update_tag_by_id)
                      .delete(delete_tag_by_id))
}
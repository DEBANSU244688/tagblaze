/// Imports tag-related handler functions for managing tags in the application.
///
/// The following handlers are imported:
/// - `create_tag`: Handles creation of a new tag.
/// - `get_tags`: Retrieves a list of all tags.
/// - `get_tag_by_id`: Fetches a tag by its unique identifier.
/// - `update_tag_by_id`: Updates an existing tag by its ID.
/// - `delete_tag_by_id`: Deletes a tag by its ID.
use crate::handlers::tag::{
    create_tag, delete_tag_by_id, get_tag_by_id, get_tags, update_tag_by_id,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_tag).get(get_tags))
        .route(
            "/{id}",
            get(get_tag_by_id)
                .put(update_tag_by_id)
                .delete(delete_tag_by_id),
        )
}

use axum::{
    routing::{post, get}, Router,
};
use crate::handlers::relations::{
    attach_tag, detach_tag, get_tags_for_ticket,
};

pub fn routes() -> Router {
    Router::new()
        .route("/{ticket_id}/tags/{tag_id}", post(attach_tag)
                                                    .delete(detach_tag))
        .route("/{ticket_id}/tags", get(get_tags_for_ticket))
}
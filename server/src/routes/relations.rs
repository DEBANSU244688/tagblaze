/// Imports relation handler functions for managing tag associations with tickets.
///
/// - `attach_tag`: Attaches a tag to a ticket.
/// - `detach_tag`: Detaches a tag from a ticket.
/// - `get_tags_for_ticket`: Retrieves all tags associated with a specific ticket.
use crate::handlers::relations::{attach_tag, detach_tag, get_tags_for_ticket};
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/{ticket_id}/tags/{tag_id}",
            post(attach_tag).delete(detach_tag),
        )
        .route("/{ticket_id}/tags", get(get_tags_for_ticket))
}

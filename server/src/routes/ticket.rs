/// Imports ticket-related handler functions from the `handlers::ticket` module.
///
/// The following functions are imported:
/// - `create_ticket`: Handles creation of new tickets.
/// - `get_tickets`: Retrieves a list of all tickets.
/// - `get_ticket_by_id`: Fetches a ticket by its unique identifier.
/// - `delete_ticket_by_id`: Deletes a ticket by its unique identifier.
/// - `update_ticket_by_id`: Updates a ticket by its unique identifier.
use crate::handlers::ticket::{
    create_ticket, delete_ticket_by_id, get_ticket_by_id, get_tickets, update_ticket_by_id,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_ticket).get(get_tickets))
        .route(
            "/{id}",
            get(get_ticket_by_id)
                .delete(delete_ticket_by_id)
                .put(update_ticket_by_id),
        )
}

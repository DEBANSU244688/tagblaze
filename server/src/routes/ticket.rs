use axum::{
    routing::{post, get}, Router,
};
use crate::handlers::ticket::{
    create_ticket, get_tickets, get_ticket_by_id, delete_ticket_by_id, update_ticket_by_id,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_ticket)
                        .get(get_tickets)) 
        .route("/{id}", get(get_ticket_by_id)
                       .delete(delete_ticket_by_id)
                       .put(update_ticket_by_id))
}
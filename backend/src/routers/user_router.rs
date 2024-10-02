use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::{handlers::user_handel::{create_user_handlers, get_user_handler}, AppState};


pub fn user_router (db : Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_user_handlers))
        .route("/:id", get(get_user_handler))
        .with_state(db)
}
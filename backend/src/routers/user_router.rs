use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{handlers::user_handel::create_user_handlers, AppState};


pub fn user_router (db : Arc<AppState>) -> Router {
    Router::new()
        .route("/", post(create_user_handlers))
        .with_state(db)
}
use std::sync::Arc;

use axum::Router;

use crate::AppState;

use super::user_router::user_router;

pub fn all_routers (db : Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/v1/user", user_router(db))
}
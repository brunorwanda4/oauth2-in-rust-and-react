use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{models::user_model::UserModel, AppState};

pub async fn create_user_handlers (
    State(app_state) : State<Arc<AppState>>,
    Json(user_model) : Json<UserModel>
) -> impl IntoResponse {
    let new_user = app_state.db.create_user(user_model).await;

    match new_user {
        Ok(res) => (StatusCode::OK , Json(res)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}
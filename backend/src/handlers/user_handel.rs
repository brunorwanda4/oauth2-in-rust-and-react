use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};

use crate::{errors::{db_error::DbError, user_result::CreateUserResult}, models::user_model::UserModel, AppState};

pub async fn create_user_handlers (
    State(app_state) : State<Arc<AppState>>,
    Json(user_model) : Json<UserModel>
) -> impl IntoResponse {
    let user_email = user_model.email.clone();

    let find_user_email = app_state.db.get_user_by_email(&user_email).await;
    if find_user_email.is_ok() {
        let error_res = CreateUserResult {
            success: false,
            message : DbError::UserEmailIsReadyExit {email : user_email}.to_string(),
        };

        return  (StatusCode::NOT_ACCEPTABLE , Json(error_res)).into_response();
    }

    let new_user = app_state.db.create_user(user_model).await;

    match new_user {
        Ok(res) => (StatusCode::OK , Json(res)).into_response(),
        Err(_) => {
            let error_res = CreateUserResult {
                success : false,
                message : DbError::CreateUserError.to_string(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR , Json(error_res)).into_response()
        }
    }
}
pub async fn get_user_handler (
    State(app_state) : State<Arc<AppState>>,
    Path(user_id) : Path<String>
) -> impl IntoResponse{

    let user = app_state.db.get_user_by_id(user_id).await;

    match user {
        Ok(user) => (StatusCode::OK , Json(user)).into_response(),
        Err(_) => {
            let error_res = CreateUserResult {
                success: false,
                message : DbError::UserNotFound.to_string(),
            };
            return (StatusCode::NOT_FOUND , Json(error_res)).into_response();
        }
    }
}
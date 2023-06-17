use axum::{http::StatusCode, response::IntoResponse, Extension};


use crate::router::router::UserState;

use super::router_utils::response_message;

pub async fn root(Extension(user): Extension<UserState>) -> impl IntoResponse {
    response_message(
        StatusCode::OK,
        format!("Name of user in Middleware: {}", user.name),
        Some(()),
    )
}

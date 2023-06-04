use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use super::router_utils::response_message;

#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

pub async fn create_user(user_data: Json<CreateUser>) -> impl IntoResponse {
    response_message(
        StatusCode::OK,
        format!("New User Created\n Email: {}\n Password:{}", user_data.email, user_data.password),
    )
}

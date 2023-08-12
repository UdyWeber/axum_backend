use axum::{http::StatusCode, response::IntoResponse};

use crate::router::router_utils::response_message;

pub async fn root() -> impl IntoResponse {
    response_message(
        StatusCode::OK,
        format!("Message: brQHQC5MXWFMyZDq9Cq91baq3UusguX6WLX/o7Ve4GNsphBFxomTFbCtcYsaiDiWxnxV8gPFI6bAzviWAp2khhPwt1soNk7zxY6OR+LcZ6kxntUAUu2KgbDTtCa8o388"),
        Some(()),
    )
}

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Response<T> {
    message: String,
    code: u16,
    #[serde(default)]
    additional_data: Option<T>,
}

pub fn response_message(
    status_code: StatusCode,
    message: String,
    additional_data: Option<impl Serialize>,
) -> impl IntoResponse {
    let response = Response {
        additional_data: additional_data,
        code: status_code.as_u16(),
        message: message,
    };

    (status_code, Json(response).into_response())
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

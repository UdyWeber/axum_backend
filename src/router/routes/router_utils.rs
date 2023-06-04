use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub fn response_message(status_code: StatusCode, message: String) -> impl IntoResponse {
    let json_message = Json(json!(
        {
            "message": message,
            "code": status_code.as_u16()
        }
    ));

    (status_code, json_message)
}

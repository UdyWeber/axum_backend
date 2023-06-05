use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use super::router_utils::response_message;

#[derive(Deserialize, Serialize, Clone)]
pub struct HelloParams {
    pub name: Option<String>,
    pub age: Option<u32>,
}

pub async fn hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let copy_params = params.clone();

    match (copy_params.age, copy_params.name) {
        (None, None) => response_message(
            StatusCode::BAD_REQUEST,
            String::from("Dont Know who to say hello to"),
            Some(params),
        ),
        (None, Some(n)) => response_message(StatusCode::OK, format!("Hello {n}"), None),
        (Some(a), None) => response_message(
            StatusCode::BAD_REQUEST,
            format!("Fuck that your age is {a}, WHATS YOUR NAME"),
            None,
        ),
        (Some(a), Some(n)) => {
            response_message(StatusCode::OK, format!("Hello {n}, your age is {a}"), None)
        }
    }
}

use axum::{response::IntoResponse, extract::Query, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::json;


#[derive(Deserialize)]
pub struct HelloParams{
    pub name: Option<String>,
    pub age: Option<u32>
}

pub async fn hello (Query(params): Query<HelloParams>) -> impl IntoResponse {
    match (params.age, params.name) {
        (None, None) => (StatusCode::BAD_REQUEST, Json(json!({"message": String::from("Dont Know who to say hello to")}))),
        (None, Some(n)) => (StatusCode::OK, Json(json!({"message": format!("Hello {n}")}))),
        (Some(a), None) => (StatusCode::BAD_REQUEST, Json(json!({"message": format!("Fuck that your age is {a}, WHATS YOUR NAME")}))),
        (Some(a), Some(n)) => (StatusCode::OK, Json(json!({"message": format!("Hello {n}, your age is {a}")}))),
    }    
}
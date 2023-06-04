use axum::{
    Router, 
    routing::{get},
};

use crate::router::routes::{
    root_route::root,
    hello_route::hello,
};

pub async fn mount_router () -> Router {
    Router::new()
    .route("/", get(root))
    .route("/hello", get(hello))
}
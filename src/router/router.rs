use axum::{routing::{get, post}, Router};

use crate::router::routes::{create_user_route::create_user, hello_route::hello, root_route::root};

pub fn mount_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/create_user", post(create_user))
}

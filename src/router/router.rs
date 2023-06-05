use axum::{
    routing::{get, post},
    Router,
};

use super::routes::testimonial_route::create_testimonial;
use crate::{
    generics::Pool,
    router::routes::{hello_route::hello, root_route::root, user_route::create_user},
};

pub fn mount_router(app_state: Pool) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/user", post(create_user))
        .route("/testimonial", post(create_testimonial))
        .with_state(app_state)
}

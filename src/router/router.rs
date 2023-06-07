use axum::{
    routing::{get, post},
    Router, middleware,
};

use super::{routes::testimonial_route::{create_testimonial, get_testimonials, get_testimonials_total_count}, guard::guard_middleware};
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
        .route("/testimonial/list", get(get_testimonials))
        .route("/testimonial/count", get(get_testimonials_total_count))
        .layer(middleware::from_fn(guard_middleware))
        .with_state(app_state)
}

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use super::{
    guard::guard_middleware,
    routes::{
        create_reaction::create_reaction, create_testimonial::create_testimonial,
        get_reaction::get_reactions, get_testimonial::get_testimonials,
        get_testimonial_count::get_testimonials_total_count,
    },
};
use crate::{generics::Pool, router::routes::root_route::root};

pub fn mount_router(app_state: Pool) -> Router {
    Router::new()
        .route("/testimonial", post(create_testimonial))
        .route("/testimonial/list", get(get_testimonials))
        .route("/testimonial/count", get(get_testimonials_total_count))
        .route("/reactions/create", post(create_reaction))
        .route("/reactions/list/:project_name", get(get_reactions))
        .layer(middleware::from_fn(guard_middleware))
        .route("/", get(root))
        .with_state(app_state)
}

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use super::{
    guard::guard_middleware,
    routes::{
        create_reaction::create_reaction,
        reaction_route::get_reactions,
        testimonial_route::{create_testimonial, get_testimonials, get_testimonials_total_count},
    },
};
use crate::{
    generics::Pool,
    router::routes::{hello_route::hello, root_route::root, user_route::create_user},
};

#[derive(Clone)]
pub struct UserState {
    pub name: String,
}

pub fn mount_router(app_state: Pool) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/user", post(create_user))
        .route("/testimonial", post(create_testimonial))
        .route("/testimonial/list", get(get_testimonials))
        .route("/testimonial/count", get(get_testimonials_total_count))
        .route("/reactions/create", post(create_reaction))
        .route("/reactions/list/:project_name", get(get_reactions))
        .layer(middleware::from_fn(guard_middleware))
        .with_state(app_state)
}

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use validator::Validate;

use crate::database::schema::testimonials::{self, project_name};
use crate::generics::Pool;
use crate::models::testimonial::{RequestTetimonials, Testimonial};
use crate::router::router_utils::{internal_error, response_message};

pub async fn get_testimonials(
    State(pool): State<Pool>,
    Json(request_data): Json<RequestTetimonials>,
) -> impl IntoResponse {
    let mut conn = pool.get_owned().await.map_err(internal_error).unwrap();

    match request_data.validate() {
        Ok(_) => (),
        Err(e) => return response_message(StatusCode::BAD_REQUEST, e.to_string(), None),
    }

    let per_page = request_data.per_page as i64;
    let offset = (request_data.page as i64 - 1) * per_page;

    let testimonial_list = testimonials::table
        .filter(project_name.eq(&request_data.project_name))
        .limit(per_page)
        .offset(offset)
        .load::<Testimonial>(&mut conn)
        .await
        .map_err(internal_error)
        .unwrap();

    response_message(
        StatusCode::OK,
        format!("Got some for project: {}", request_data.project_name),
        Some(testimonial_list),
    )
}

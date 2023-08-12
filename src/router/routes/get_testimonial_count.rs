use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use validator::Validate;

use crate::{
    database::schema::testimonials::{self, project_name},
    generics::Pool,
    models::{generic::PaginationData, testimonial::RequestTetimonials},
    router::router_utils::{internal_error, response_message},
};

pub async fn get_testimonials_total_count(
    State(pool): State<Pool>,
    Json(request_data): Json<RequestTetimonials>,
) -> impl IntoResponse {
    let mut conn = pool.get_owned().await.map_err(internal_error).unwrap();

    match request_data.validate() {
        Ok(_) => (),
        Err(e) => return response_message(StatusCode::BAD_REQUEST, e.to_string(), None),
    }

    let per_page = request_data.per_page as i64;

    let total_count = testimonials::table
        .count()
        .filter(project_name.eq(&request_data.project_name))
        .limit(per_page)
        .get_result::<i64>(&mut conn)
        .await
        .map_err(internal_error)
        .unwrap();

    let pagination_data = PaginationData::new(request_data.page as i64, per_page, total_count);

    response_message(
        StatusCode::OK,
        format!("Total Count"),
        Some(pagination_data),
    )
}

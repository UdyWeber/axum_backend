use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::{SelectableHelper, QueryDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;
use validator::Validate;

use super::router_utils::{internal_error, response_message};
use crate::generics::Pool;
use crate::models::{CreateTestimonial, Testimonial, RequestTetimonials, PaginationData};
use crate::schema::testimonials::{self, project_name};

pub async fn create_testimonial(
    State(pool): State<Pool>,
    Json(testimonial_data): Json<CreateTestimonial>,
) -> impl IntoResponse {
    let mut conn = pool.get_owned().await.map_err(internal_error).unwrap();

    match testimonial_data.validate() {
        Ok(_) => (),
        Err(e) => return response_message(
            StatusCode::BAD_REQUEST, 
            e.to_string(), 
            None
        ),
    }

    let response = diesel::insert_into(testimonials::table)
        .values(testimonial_data)
        .returning(Testimonial::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)
        .unwrap();

    response_message(
        StatusCode::CREATED,
        format!("Testimonial created successfully."),
        Some(response),
    )
}


pub async fn get_testimonials(
    State(pool): State<Pool>,
    Json(request_data): Json<RequestTetimonials>
) -> impl IntoResponse {
    let mut conn = pool.get_owned().await.map_err(internal_error).unwrap();

    match request_data.validate() {
        Ok(_) => (),
        Err(e) => return response_message(
            StatusCode::BAD_REQUEST, 
            e.to_string(), 
            None
        ),
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

pub async fn get_testimonials_total_count(
    State(pool): State<Pool>,
    Json(request_data): Json<RequestTetimonials>
) -> impl IntoResponse {
    let mut conn = pool.get_owned().await.map_err(internal_error).unwrap();

    match request_data.validate() {
        Ok(_) => (),
        Err(e) => return response_message(
            StatusCode::BAD_REQUEST, 
            e.to_string(), 
            None
        ),
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

    let pagination_data = PaginationData::new(
        request_data.page as i64, 
        per_page, 
        total_count
    );

    response_message(
        StatusCode::OK,
        format!("Total Count"),
        Some(pagination_data),
    )
}
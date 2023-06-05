use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use validator::Validate;

use super::router_utils::{internal_error, response_message};
use crate::generics::Pool;
use crate::models::{CreateTestimonial, Testimonial};
use crate::schema::testimonials;

pub async fn create_testimonial(
    State(pool): State<Pool>,
    Json(testimonial_data): Json<CreateTestimonial>,
) -> impl IntoResponse {
    let mut conn = pool.get_owned().await.map_err(internal_error).unwrap();

    match testimonial_data.validate() {
        Ok(_) => {
            // Integrate DB HERE
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
        Err(e) => return response_message(StatusCode::BAD_REQUEST, e.to_string(), None),
    }
}

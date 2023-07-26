use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::schema::{
    reactions::{self},
    testimonials::{self},
};

#[derive(Serialize, Selectable, Queryable, Debug)]
pub struct Testimonial {
    id: Uuid,
    comment: String,
    commenter: String,
    project_name: String,
}

#[derive(Deserialize, Validate, Insertable)]
#[diesel(table_name = testimonials)]
pub struct CreateTestimonial {
    #[validate(length(min = 1))]
    pub comment: String,
    #[validate(length(min = 1))]
    pub commenter: String,
    #[validate(length(min = 1))]
    pub project_name: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestTetimonials {
    #[validate(custom = "validate_pagination")]
    pub page: i8,
    #[validate(custom = "validate_pagination")]
    pub per_page: i8,
    #[validate(custom = "validate_string")]
    pub project_name: String,
}

pub fn validate_pagination(number: i8) -> Result<(), ValidationError> {
    match number >= 1 {
        true => Ok(()),
        false => Err(ValidationError::new(
            "Pagination value must be greather than or equal to 1",
        )),
    }
}

pub fn validate_string(string_to_validate: &str) -> Result<(), ValidationError> {
    let string_to_validate = string_to_validate.trim().replace(" ", "");

    match string_to_validate.len() >= 1 || !string_to_validate.is_empty() {
        true => Ok(()),
        false => Err(ValidationError::new("Value is not a valid String")),
    }
}

#[derive(Serialize, Deserialize)]
pub struct PaginationData {
    page: i64,
    per_page: i64,
    pages: i64,
    total_count: i64,
}

impl PaginationData {
    pub fn new(page: i64, per_page: i64, total_count: i64) -> Self {
        let pages = ((total_count as f64 / per_page as f64).ceil()) as i64;

        PaginationData {
            page: page,
            per_page: per_page,
            pages: pages,
            total_count: total_count,
        }
    }
}

#[derive(Serialize, Selectable, Queryable, Debug)]
pub struct Reaction {
    pub id: Uuid,
    pub project_name: String,
    pub reaction_asset: String,
    pub reacter_unique_id: String,
}

#[derive(Validate, Deserialize, Insertable)]
#[diesel(table_name = reactions)]
pub struct CreateReaction {
    #[validate(custom = "validate_string")]
    pub reaction_asset: String,
    #[validate(custom = "validate_string")]
    pub reacter_unique_id: String,
    #[validate(custom = "validate_string")]
    pub project_name: String,
}

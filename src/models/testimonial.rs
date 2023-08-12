use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::database::schema::testimonials;

use super::super::utils::validators::{validate_pagination, validate_string};

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

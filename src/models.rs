use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::testimonials::{self};

#[derive(Serialize, Selectable, Queryable, Debug)]
pub struct Testimonial {
    uuid: i32,
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

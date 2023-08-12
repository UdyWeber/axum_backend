use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::database::schema::reactions;

use super::super::utils::validators::validate_string;

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

#[derive(Deserialize, Serialize, Validate)]
pub struct GetReactions {
    #[validate(custom = "validate_string")]
    pub project_name: String,
}

#[derive(Serialize)]
pub struct ReactionItem {
    pub reaction_asset: String,
    pub counter: i64,
}

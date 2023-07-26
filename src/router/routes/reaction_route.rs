use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use diesel::{dsl::count, ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use validator::Validate;
use rayon::prelude::*;

use super::router_utils::{internal_error, response_message};
use crate::{generics::Pool, models::validate_string, schema::reactions};

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

pub async fn get_reactions(
    State(pool): State<Pool>,
    Path(request_data): Path<GetReactions>,
) -> impl IntoResponse {
    match request_data.validate() {
        Ok(_) => (),
        Err(e) => return response_message(StatusCode::BAD_REQUEST, e.to_string(), None),
    }

    let mut connection = pool.get_owned().await.map_err(internal_error).unwrap();

    let count_with_asset: Vec<(i64, String)> = reactions::table
        .filter(reactions::project_name.eq(&request_data.project_name))
        .group_by(reactions::reaction_asset)
        .select((count(reactions::id), reactions::reaction_asset))
        .load::<(i64, String)>(&mut connection)
        .await
        .map_err(internal_error)
        .unwrap_or(vec![]);

    let reactions: Vec<ReactionItem> = count_with_asset.into_par_iter()
        .map(|(count, asset)| ReactionItem {
            counter: count,
            reaction_asset: asset,
        })
        .collect();

    response_message(
        StatusCode::OK,
        format!("Reactions for project {}", request_data.project_name),
        Some(reactions),
    )
}

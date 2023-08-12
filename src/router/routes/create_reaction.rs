use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use validator::Validate;

use crate::{
    database::schema::reactions,
    generics::Pool,
    models::reaction::{CreateReaction, Reaction},
    router::router_utils::{internal_error, response_message},
};

pub async fn create_reaction(
    State(pool): State<Pool>,
    Json(reaction_data): Json<CreateReaction>,
) -> impl IntoResponse {
    match reaction_data.validate() {
        Ok(_) => (),
        Err(e) => return response_message(StatusCode::BAD_REQUEST, e.to_string(), None),
    }

    let mut connection = pool.get_owned().await.map_err(internal_error).unwrap();

    match reactions::table
        .select(Reaction::as_select())
        .filter(reactions::project_name.eq(&reaction_data.project_name))
        .filter(reactions::reacter_unique_id.eq(&reaction_data.reacter_unique_id))
        .filter(reactions::reaction_asset.eq(&reaction_data.reaction_asset))
        .first::<Reaction>(&mut connection)
        .await
    {
        Ok(reaction) => {
            diesel::delete(reactions::table.filter(reactions::id.eq(reaction.id)))
                .execute(&mut connection)
                .await
                .map_err(internal_error)
                .unwrap();

            response_message(
                StatusCode::OK,
                format!(
                    "Removed reaction {} for project {}",
                    reaction_data.reaction_asset, reaction_data.project_name
                ),
                None,
            )
        }
        Err(_) => {
            let inserted_reaction = diesel::insert_into(reactions::table)
                .values(&reaction_data)
                .get_result::<Reaction>(&mut connection)
                .await
                .map_err(internal_error)
                .unwrap();

            return response_message(
                StatusCode::OK,
                format!(
                    "Created reaction {} for project {}",
                    reaction_data.reaction_asset, reaction_data.project_name
                ),
                Some(inserted_reaction),
            );
        }
    }
}

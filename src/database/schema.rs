// @generated automatically by Diesel CLI.

diesel::table! {
    reactions (id) {
        id -> Uuid,
        reaction_asset -> Varchar,
        reacter_unique_id -> Varchar,
        project_name -> Varchar,

    }
}

diesel::table! {
    testimonials (id) {
        id -> Uuid,
        comment -> Varchar,
        commenter -> Varchar,
        project_name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(reactions, testimonials,);

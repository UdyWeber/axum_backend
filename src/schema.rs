// @generated automatically by Diesel CLI.

diesel::table! {
    testimonials (uuid) {
        uuid -> Int4,
        comment -> Varchar,
        commenter -> Varchar,
        project_name -> Varchar,
    }
}

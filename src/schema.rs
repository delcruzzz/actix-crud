// @generated automatically by Diesel CLI.

diesel::table! {
    tweet (id) {
        id -> Int4,
        message -> Varchar,
        created_at -> Timestamp,
    }
}

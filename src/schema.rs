// @generated automatically by Diesel CLI.

diesel::table! {
    adverts (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        photo -> Varchar,
        price -> Money,
        created_at -> Timestamptz,
    }
}

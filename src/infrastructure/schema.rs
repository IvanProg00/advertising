// @generated automatically by Diesel CLI.

diesel::table! {
    adverts (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Varchar,
        description -> Text,
        #[max_length = 200]
        photo -> Varchar,
        price -> Money,
        created_at -> Timestamptz,
    }
}

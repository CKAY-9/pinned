// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        oauth_id -> Text,
        username -> Text,
        avatar -> Text,
        bio -> Text,
        token -> Text,
        collections -> Array<Int4>,
    }
}

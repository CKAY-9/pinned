// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        avatar -> Text,
        bio -> Text,
        token -> Text,
        collections -> Array<Int4>,
    }
}

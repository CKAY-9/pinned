// @generated automatically by Diesel CLI.

diesel::table! {
    collections (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        linked_posts -> Array<Int4>,
        linked_comments -> Array<Int4>,
        recommended_collections -> Array<Int4>,
        creator -> Int4,
        likes -> Array<Int4>,
        dislikes -> Array<Int4>,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        post -> Int4,
        creator -> Int4,
        content -> Text,
        posted -> Text,
        likes -> Array<Int4>,
        dislikes -> Array<Int4>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        file_id -> Text,
        description -> Text,
        posted -> Text,
        creator -> Int4,
        likes -> Array<Int4>,
        dislikes -> Array<Int4>,
        comments -> Array<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        oauth_id -> Text,
        username -> Text,
        avatar -> Text,
        bio -> Text,
        token -> Text,
        joined -> Text,
        collections -> Array<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    collections,
    comments,
    posts,
    users,
);

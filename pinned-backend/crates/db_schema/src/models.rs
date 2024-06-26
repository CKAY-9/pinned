use diesel::prelude::*;
use serde::{ Deserialize, Serialize };

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub oauth_id: String,
    pub username: String,
    pub avatar: String,
    pub bio: String,
    pub token: String,
    pub joined: String,
    pub collections: Vec<i32>,
    pub favourites: Vec<i32>,
    pub pinned: Vec<i32>
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub token: String,
    pub oauth_id: String,
    pub bio: String,
    pub joined: String,
    pub username: String,
    pub avatar: String,
    pub collections: Vec<i32>,
    pub favourites: Vec<i32>,
    pub pinned: Vec<i32>
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub file_id: String,
    pub description: String,
    pub posted: String,
    pub creator: i32,
    pub likes: Vec<i32>,
    pub dislikes: Vec<i32>,
    pub comments: Vec<i32>,
}

#[derive(Insertable, Debug, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub file_id: String,
    pub comments: Vec<i32>,
    pub title: String,
    pub description: String,
    pub posted: String,
    pub creator: i32,
    pub likes: Vec<i32>,
    pub dislikes: Vec<i32>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub post: i32,
    pub creator: i32,
    pub content: String,
    pub posted: String,
    pub likes: Vec<i32>,
    pub dislikes: Vec<i32>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment {
    pub post: i32,
    pub creator: i32,
    pub content: String,
    pub posted: String,
    pub likes: Vec<i32>,
    pub dislikes: Vec<i32>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::collections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub linked_posts: Vec<i32>,
    pub linked_comments: Vec<i32>,
    pub recommended_collections: Vec<i32>,
    pub creator: i32,
    pub collaborators: Vec<i32>,
    pub likes: Vec<i32>,
    pub dislikes: Vec<i32>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::collections)]
pub struct NewCollection {
    pub name: String,
    pub description: String,
    pub linked_posts: Vec<i32>,
    pub linked_comments: Vec<i32>,
    pub recommended_collections: Vec<i32>,
    pub creator: i32,
    pub collaborators: Vec<i32>,
    pub likes: Vec<i32>,
    pub dislikes: Vec<i32>,
}

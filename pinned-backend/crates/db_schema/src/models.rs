use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub collections: Vec<i32>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub token: String,
    pub oauth_id: String,
    pub bio: String,
    pub username: String,
    pub avatar: String,
    pub collections: Vec<i32>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub file_id: String,
    pub description: String,
    pub creator: i32,
    pub likes: Vec<i32>,
    pub dislikes: Vec<i32>,
    pub comments: Vec<i32>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub file_id: String,
    pub description: String,
    pub creator: i32,
    pub likes: Vec<i32>,
    pub dislikes: Vec<i32>,
    pub comments: Vec<i32>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub creator: i32,
    pub content: String,
    pub likes: Vec<i32>,
    pub dislikes: Vec<i32>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment {
    pub creator: i32,
    pub content: String,
    pub likes: Vec<i32>,
    pub dislikes: Vec<i32>
}

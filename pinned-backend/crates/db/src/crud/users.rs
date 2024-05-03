use std::vec;

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use pinned_db_schema::{models::{Collection, Comment, NewUser, Post, User}, schema::{collections, comments, posts, users}};

use crate::create_connection;

pub fn get_user_from_token(token: String) -> Option<User> {
    let connection = &mut create_connection();
    let user_result: QueryResult<User> = users::table
        .filter(users::token.eq(token))
        .first::<User>(connection);

    match user_result {
        Ok(u) => Some(u),
        Err(_e) => None
    }
}

pub fn get_user_from_id(user_id: i32) -> Option<User> {
    let connection = &mut create_connection();
    let user_result = users::table.find(user_id).first::<User>(connection);

    match user_result {
        Ok(u) => Some(u),
        Err(_e) => None,
    }
}

pub fn get_user_posts_from_id(user_id: i32) -> Vec<Post> {
    let connection = &mut create_connection();
    let posts = posts::table
        .filter(posts::creator.eq(user_id))
        .select(Post::as_select())
        .load(connection);
    if posts.is_err() {
        return vec![];
    }
    posts.unwrap()
}

pub fn get_user_collections_from_id(user_id: i32) -> Vec<Collection> {
    let connection = &mut create_connection();
    let collections = collections::table
        .filter(collections::creator.eq(user_id))
        .select(Collection::as_select())
        .load(connection);
    if collections.is_err() {
        return vec![];
    }
    collections.unwrap()
}

pub fn get_user_comments_from_id(user_id: i32) -> Vec<Comment> {
    let connection = &mut create_connection();
    let comments = comments::table
        .filter(comments::creator.eq(user_id))
        .select(Comment::as_select())
        .load(connection);
    if comments.is_err() {
        return vec![];
    }
    comments.unwrap()
}

pub fn update_user_from_id(user_id: i32, update: NewUser) -> Option<User> {
    let connection = &mut create_connection();
    let user_update = diesel::update(users::table)
        .filter(users::id.eq(user_id))
        .set(update)
        .get_result::<User>(connection);

    match user_update {
        Ok(u) => Some(u),
        Err(_e) => None,
    }
}
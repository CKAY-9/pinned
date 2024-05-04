use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use pinned_db_schema::{models::{NewPost, Post}, schema::posts};

use crate::create_connection;

pub fn create_post(new_post: NewPost) -> Option<Post> {
    let connection = &mut create_connection();
    let insert = diesel
        ::insert_into(posts::table)
        .values(new_post)
        .get_result::<Post>(connection);
    match insert {
        Ok(p) => { Some(p) }
        Err(_e) => { None }
    }
}

pub fn get_post_from_id(post_id: i32) -> Option<Post> {
    let connection = &mut create_connection();
    let post_result = posts::table.find(post_id).first::<Post>(connection);

    match post_result {
        Ok(p) => Some(p),
        Err(_e) => None,
    }
}

pub fn update_post_from_id(post_id: i32, update: NewPost) -> Option<Post> {
    let connection = &mut create_connection();
    let post_update = diesel::update(posts::table)
        .filter(posts::id.eq(post_id))
        .set(update)
        .get_result::<Post>(connection);

    match post_update {
        Ok(p) => Some(p),
        Err(_e) => None,
    }
}

pub fn delete_post_from_id(post_id: i32) -> bool {
    let connection = &mut create_connection();
    let post_delete = diesel
        ::delete(posts::table)
        .filter(posts::id.eq(post_id))
        .execute(connection);

    match post_delete {
        Ok(_d) => true,
        Err(_e) => false,
    }
}

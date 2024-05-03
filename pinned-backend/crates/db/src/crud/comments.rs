use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use pinned_db_schema::{models::{Comment, NewComment}, schema::comments};

use crate::create_connection;

pub fn create_comment(new_comment: NewComment) -> Option<Comment> {
    let connection = &mut create_connection();
    let insert = diesel
        ::insert_into(comments::table)
        .values(new_comment)
        .get_result::<Comment>(connection);
    match insert {
        Ok(c) => { Some(c) }
        Err(_e) => { None }
    }
}

pub fn get_comment_from_id(comment_id: i32) -> Option<Comment> {
    let connection = &mut create_connection();
    let comment_result = comments::table.find(comment_id).first::<Comment>(connection);

    match comment_result {
        Ok(c) => Some(c),
        Err(_e) => None,
    }
}

pub fn update_comment_from_id(comment_id: i32, update: NewComment) -> Option<Comment> {
    let connection = &mut create_connection();
    let comment_update = diesel::update(comments::table)
        .filter(comments::id.eq(comment_id))
        .set(update)
        .get_result::<Comment>(connection);

    match comment_update {
        Ok(c) => Some(c),
        Err(_e) => None,
    }
}

pub fn delete_comment_from_id(comment_id: i32) -> bool {
    let connection = &mut create_connection();
    let comment_delete = diesel
        ::delete(comments::table)
        .filter(comments::id.eq(comment_id))
        .execute(connection);

    match comment_delete {
        Ok(_d) => true,
        Err(_e) => false,
    }
}
use diesel::{ ExpressionMethods, QueryDsl, RunQueryDsl };
use pinned_db_schema::{ models::{ Collection, NewCollection }, schema::collections };

use crate::create_connection;

pub fn create_collection(new_collection: NewCollection) -> Option<Collection> {
    let connection = &mut create_connection();
    let insert = diesel
        ::insert_into(collections::table)
        .values(new_collection)
        .get_result::<Collection>(connection);
    match insert {
        Ok(c) => { Some(c) }
        Err(_e) => { 
            println!("{}", _e);
            None
        }
    }
}

pub fn get_collection_from_id(collection_id: i32) -> Option<Collection> {
    let connection = &mut create_connection();
    let collection_result = collections::table.find(collection_id).first::<Collection>(connection);

    match collection_result {
        Ok(collection) => Some(collection),
        Err(_e) => None,
    }
}

pub fn update_collection_from_id(collection_id: i32, update: NewCollection) -> Option<Collection> {
    let connection = &mut create_connection();
    let collection_update = diesel
        ::update(collections::table)
        .filter(collections::id.eq(collection_id))
        .set(update)
        .get_result::<Collection>(connection);

    match collection_update {
        Ok(c) => Some(c),
        Err(_e) => None,
    }
}

pub fn delete_collection_from_id(collection_id: i32) -> bool {
    let connection = &mut create_connection();
    let collection_delete = diesel
        ::delete(collections::table)
        .filter(collections::id.eq(collection_id))
        .execute(connection);

    match collection_delete {
        Ok(_d) => true,
        Err(_e) => false,
    }
}

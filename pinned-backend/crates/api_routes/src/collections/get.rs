use actix_web::{ get, Responder, web, HttpResponse };
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use pinned_db::{create_connection, crud::{collections::get_collection_from_id, users::get_user_from_id}};
use pinned_db_schema::{models::{Collection, User}, schema::collections};
use rand::{seq::IteratorRandom, thread_rng};
use reqwest::StatusCode;
use super::dto::CollectionExploreMessage;
use crate::{ collections::dto::{ GetCollaboratorsMessage, GetCollectionDTO, GetCollectionMessage }, dto::Message, posts::dto::SearchPostsDTO };

#[get("")]
pub async fn get_collection(
    query: web::Query<GetCollectionDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let collection = get_collection_from_id(query.collection_id);
    match collection {
        Some(c) =>
            Ok(
                HttpResponse::Ok().json(GetCollectionMessage {
                    message: "Got collection".to_string(),
                    collection: c,
                })
            ),
        None =>
            Ok(
                HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
                    message: "Failed to get collection".to_string(),
                })
            ),
    }
}

#[get("/collaborators")]
pub async fn get_collection_collaborators(
    query: web::Query<GetCollectionDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let collection_option = get_collection_from_id(query.collection_id);
    if collection_option.is_none() {
        return Ok(HttpResponse::Ok().status(StatusCode::NOT_FOUND).json(Message {
            message: "Failed to get collection".to_string()
        }));
    }

    let collection = collection_option.unwrap();
    let mut users: Vec<User> = vec![];
    for collab_id in collection.collaborators {
        let temp_user = get_user_from_id(collab_id);
        if temp_user.is_none() {
            continue;
        }
        users.push(temp_user.unwrap());
    }

    Ok(HttpResponse::Ok().json(GetCollaboratorsMessage {
        message: "Got collaborators".to_string(),
        collaborators: users
    }))
}

#[get("/explore")]
pub async fn get_explore_collections() -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection: &mut diesel::prelude::PgConnection = &mut create_connection();
    let collections_result: QueryResult<Vec<Collection>> = collections::table
        .select(Collection::as_select())
        .load(connection);
    if collections_result.is_err() {
        return Ok(
            HttpResponse::Ok().json(Message {
                message: "Failed to get collections".to_string(),
            })
        );
    }

    let max_return = 10;
    let all_collections = collections_result.expect("Failed to get collections");

    if all_collections.iter().count() <= max_return {
        return Ok(
            HttpResponse::Ok().json(CollectionExploreMessage {
                message: "Got collections".to_string(),
                collections: all_collections,
            })
        );
    }

    let mut rng = thread_rng();
    let cs: Vec<Collection> = all_collections.into_iter().choose_multiple(&mut rng, max_return);

    Ok(
        HttpResponse::Ok().json(CollectionExploreMessage {
            message: "Got collections".to_string(),
            collections: cs,
        })
    )
}

#[get("/search")]
pub async fn search_collections(
    query: web::Query<SearchPostsDTO>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let connection = &mut create_connection();
    let mut collections_vec: Vec<Collection> = Vec::new();

    if query.post_id != 0 {
        let collection_result: QueryResult<Collection> = collections::table
            .find(query.post_id)
            .limit(15)
            .first::<Collection>(connection);
        if collection_result.is_ok() {
            let collection = collection_result.unwrap();
            collections_vec.push(collection);
        }
    }

    let all_posts_result: QueryResult<Vec<Collection>> = collections::table.load(connection);

    if all_posts_result.is_ok() {
        let all_posts = all_posts_result.unwrap();
        let mut index = 0;
        for collection in all_posts {
            if index > 15 {
                break;
            }
            if collection.name.contains(query.name.as_str()) {
                collections_vec.push(collection);
            }
            index += 1;
        }
    }

    Ok(
        HttpResponse::Ok().json(CollectionExploreMessage {
            message: "Fetched collections".to_string(),
            collections: collections_vec,
        })
    )
}
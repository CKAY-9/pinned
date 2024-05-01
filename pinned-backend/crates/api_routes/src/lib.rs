use actix_web::web;

pub mod comments;
pub mod dto;
pub mod posts;
pub mod users;
pub mod collections;

use collections::{
    post::create_new_collection, 
    get::get_collection, 
    put::{
        update_add_to_collection, 
        update_collection,
        update_likes_on_collection
    },
    delete::delete_collection
};
use comments::{
    delete::delete_comment, 
    get::get_comment, 
    post::create_new_comment, 
    put::update_likes_on_comment
};
use posts::{
    delete::delete_post,
    get::{
        get_explore_posts, get_post, get_today_pinned
    },
    post::create_new_post,
    put::{
        update_likes_on_post, update_post
    },
};
use users::{
    delete::delete_user,
    get::{
        get_account, get_discord_user_authentication, get_explore_users, get_github_user_authentication, get_profile, get_search_users, get_user_collections, get_users_comments, get_users_posts
    },
    post::post_reset_user,
};

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_account) // authorization header - token
            .service(get_profile) // id parameter - >0 user id
            .service(delete_user) // authorization header - token
            .service(get_search_users) // id and username parameter
            .service(post_reset_user) // authorization header
            .service(get_users_posts) // user id parameter
            .service(get_user_collections) // user id parameter
            .service(get_users_comments) // user id parameter
            .service(get_explore_users)
            .service(
                web::scope("/auth")
                    .service(get_discord_user_authentication) // code parameter - provided by oauth
                    .service(get_github_user_authentication), // code parameter - provided by oauth
            ),
    );
}

pub fn configure_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(create_new_post) // auth header
            .service(get_post) // post_id parameter
            .service(delete_post) // auth header, post_id
            .service(update_post) // auth header, post_id, title, descripition body
            .service(get_today_pinned) // no inputs 
            .service(update_likes_on_post) // post_id, type: -1 = dislike, 0 = reset, 1 = like, auth header
            .service(get_explore_posts)
    );
}

pub fn configure_comment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/comments")
            .service(create_new_comment) // auth header, new post dto
            .service(get_comment) // comment_id query
            .service(delete_comment) // auth header, comment_id data
            .service(update_likes_on_comment) // auth header, comment_id, like_type data 
    );
}

pub fn configure_collections_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/collections")
            .service(create_new_collection) // name, description data, auth header
            .service(get_collection) // collection_id parameter
            .service(update_add_to_collection) // collection_id, post_id data, auth header
            .service(update_collection) // name, description, collection_id data, auth header
            .service(delete_collection) // collection_id data, auth header
            .service(update_likes_on_collection)
    );
}

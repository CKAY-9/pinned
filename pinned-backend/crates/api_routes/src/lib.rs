use actix_web::web;

pub mod comments;
pub mod dto;
pub mod posts;
pub mod users;
pub mod collections;

use collections::{
    delete::delete_collection,
    get::{ get_collection, get_collection_collaborators },
    post::create_new_collection,
    put::{
        add_collaborator_to_collection, remove_collaborator_from_collection, update_add_to_collection, update_collection, update_likes_on_collection
    },
};
use comments::{
    delete::delete_comment,
    get::get_comment,
    post::create_new_comment,
    put::update_likes_on_comment,
};
use posts::{
    delete::delete_post,
    get::{ get_explore_posts, get_post, get_today_pinned, search_posts },
    post::create_new_post,
    put::{ update_likes_on_post, update_post },
};
use users::{
    delete::delete_user,
    get::{
        get_account,
        get_discord_user_authentication,
        get_explore_users,
        get_github_user_authentication,
        get_profile,
        get_search_users,
        get_user_collections,
        get_users_comments,
        get_users_posts,
    },
    post::post_reset_user,
};

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/users")
            .service(get_account)
            .service(get_profile)
            .service(delete_user)
            .service(get_search_users)
            .service(post_reset_user)
            .service(get_users_posts)
            .service(get_user_collections)
            .service(get_users_comments)
            .service(get_explore_users)
            .service(
                web
                    ::scope("/auth")
                    .service(get_discord_user_authentication)
                    .service(get_github_user_authentication)
            )
    );
}

pub fn configure_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/posts")
            .service(create_new_post)
            .service(get_post)
            .service(delete_post)
            .service(update_post)
            .service(get_today_pinned)
            .service(update_likes_on_post)
            .service(get_explore_posts)
            .service(search_posts)
    );
}

pub fn configure_comment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/comments")
            .service(create_new_comment)
            .service(get_comment)
            .service(delete_comment)
            .service(update_likes_on_comment)
    );
}

pub fn configure_collections_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/collections")
            .service(create_new_collection)
            .service(get_collection_collaborators)
            .service(get_collection)
            .service(update_add_to_collection)
            .service(update_collection)
            .service(delete_collection)
            .service(update_likes_on_collection)
            .service(add_collaborator_to_collection)
            .service(remove_collaborator_from_collection)
    );
}

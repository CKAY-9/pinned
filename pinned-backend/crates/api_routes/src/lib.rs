use actix_web::web;

pub mod users;
pub mod posts;
pub mod comments;
pub mod dto;

use comments::{post::create_new_comment, get::get_comment};
use users::{get::{
    get_discord_user_authentication, 
    get_github_user_authentication, 
    get_account, 
    get_profile, get_search_users
}, delete::delete_user, post::post_reset_user};
use posts::{post::create_new_post, get::get_post};

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_account) // authorization header - token
            .service(get_profile) // id parameter - >0 user id
            .service(delete_user) // authorization header - token
            .service(get_search_users) // id and username parameter
            .service(post_reset_user) // authorization header
            .service(
                web::scope("/auth")
                    .service(get_discord_user_authentication) // code parameter - provided by oauth
                    .service(get_github_user_authentication) // code parameter - provided by oauth
            )
    );
}

pub fn configure_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(create_new_post) // authorization header - token
            .service(get_post) // id paramater - >0 post id
    );
}

pub fn configure_comment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/comments")
            .service(create_new_comment)
            .service(get_comment)
    );
}

use actix_web::web;

pub mod users;
pub mod posts;
pub mod dto;

use users::{get::{
    get_discord_user_authentication, 
    get_github_user_authentication, 
    get_account, 
    get_profile, get_search_users
}, delete::delete_user, post::post_reset_user};
use posts::post::{
    create_new_post, 
    get_post
};

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_account) // authorization header - token
            .service(get_profile) // id parameter - >0 user id
            .service(delete_user) // authorization header - token
            .service(get_search_users)
            .service(post_reset_user)
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

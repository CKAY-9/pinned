use actix_web::web;

pub mod users;
pub mod posts;

use users::get::{discord_user_authentication, github_user_authentication, google_user_authentication};
use posts::post::{create_new_post, get_post};

pub fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(
                web::scope("/auth")
                    .service(discord_user_authentication)       
                    .service(github_user_authentication)
                    .service(google_user_authentication)
            )
    );
}

pub fn configure_post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(create_new_post)
            .service(get_post)
    );
}

use actix_web::web;
use pinned_api_routes::{
    configure_comment_routes,
    configure_post_routes,
    configure_user_routes,
    configure_collections_routes,
};

// setup broad api routes, see api_routes crates
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web
            ::scope("/api/v1")
            .configure(configure_user_routes)
            .configure(configure_post_routes)
            .configure(configure_comment_routes)
            .configure(configure_collections_routes)
    );
}

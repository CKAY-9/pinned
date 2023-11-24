use actix_web::{App, HttpServer};
use pinned_api::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(configure)
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}

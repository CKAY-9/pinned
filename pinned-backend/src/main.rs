use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use pinned_api::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new().wrap(cors).configure(configure)
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}

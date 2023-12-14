use actix_web::{App, HttpServer};
use pinned_api::configure;
use dotenv::dotenv;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 

    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .configure(configure)
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}

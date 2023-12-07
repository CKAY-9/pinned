use actix_web::{App, HttpServer};
use pinned_api::configure;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 

    HttpServer::new(|| {
        App::new()
            .configure(configure)
    })
    .bind(("0.0.0.0", 3001))?
    .run()
    .await
}

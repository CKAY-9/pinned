use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use pinned_utils::get_env_var;

pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url: String = get_env_var("DATABASE_URL");
    PgConnection::establish(database_url.as_str())
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

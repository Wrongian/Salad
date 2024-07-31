pub mod connection;
pub mod follow;
pub mod image;
pub mod insight;
pub mod link;
pub mod notifications;
pub mod reset;
pub mod user;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::env;

pub async fn mock_connection() -> PgConnection {
    dotenv().expect("No .env file found");

    let database_url = env::var("DATABASE_URL").expect("No database url found");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

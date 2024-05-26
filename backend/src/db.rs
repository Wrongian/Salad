pub mod user;
// use dotenv::dotenv;
// use sqlx::Postgres;
use std::env;
// use sqlx::Pool;
use dotenvy::dotenv;
// connection pooling later
use diesel::prelude::*;
use diesel::result::Error;

pub async fn start_connection() -> PgConnection {
    dotenv().expect("No .env file found");

    let database_url = env::var("DATABASE_URL").expect("No database url found");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

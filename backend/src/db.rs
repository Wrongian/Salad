pub mod link;
pub mod user;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use dotenvy::dotenv;
use std::env;

pub type DBConnection = PooledConnection<ConnectionManager<PgConnection>>;

// connection pooling later

// function to start the db connection
// do not use this function to start connections with the database in each route
#[deprecated]
pub async fn start_connection() -> PgConnection {
    dotenv().expect("No .env file found");

    let database_url = env::var("DATABASE_URL").expect("No database url found");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub mod buckets;
pub mod funcs;
pub mod models;
pub mod routes;
pub mod schema;
pub mod tests;
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use buckets::file::setup_buckets;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use routes::auth::{login, logout, register, is_logged_in};
use routes::profile_controller::get_profile;
use std::env;
pub mod db;
use aws_sdk_s3::{self as s3, config};
use dotenvy::dotenv;
use http_types::headers::HeaderValue;
use std::sync::Arc;
use tide::security::{CorsMiddleware, Origin};

// Migration to DB tables creation
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// app state
pub type TidePool = Pool<ConnectionManager<PgConnection>>;
pub struct TideState {
    pub tide_pool: TidePool,
    pub s3_client: s3::Client,
}

// todo replace unwraps with expect

// main function
#[::tokio::main]
async fn main() -> tide::Result<()> {
    // load dotenv
    dotenv().expect("No .env file found");

    // setup migrations
    let mut conn = db::start_connection().await;
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    // setup aws s3 client
    let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-2");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let s3_client = s3::Client::new(&config);

    // configure s3 buckets
    setup_buckets(
        &s3_client,
        env::var("AWS_REGION")
            .expect("No aws region found")
            .as_str(),
    )
    .await;

    // App State
    // Diesel
    let database_url = env::var("DATABASE_URL").expect("No database url found");
    let pool_manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = Pool::builder()
        .build(pool_manager)
        .expect("Failed to build connection pool");
    let tide_state = Arc::new(TideState {
        tide_pool: pool,
        s3_client,
    });

    // create app
    let mut app = tide::with_state(tide_state);

    // CORS middleware
    let whitelist_urls = env::var::<&str>("CORS_WHITELIST_URLS")
        .unwrap()
        .split(",")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS, PUT".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from(whitelist_urls.clone()))
        .allow_credentials(false);
    app.with(cors);

    log::info!(
        "accepting requests from the following urls: {:?}",
        whitelist_urls
    );

    // session middleware
    // words from the documentation
    // DO NOT USE MEMORY STORE IN PRODUCTION USE A PROPER EXTERNAL DATASTORE
    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        env::var("TIDE_SECRET")
            .expect("Tide Key not found")
            .as_bytes(),
    ));

    // set up logging middleware, default log level is 'info'
    femme::start();
    app.with(tide::log::LogMiddleware::new());

    // setup routes

    // auth
    app.at("/login").post(login);
    app.at("/register").post(register);
    app.at("/logout").post(logout);
    app.at("/logged-in").get(is_logged_in);
    // profile
    app.at("/profiles/:username").get(get_profile);
    // attach to IP and port
    app.listen(funcs::get_url()).await?;

    // return
    Ok(())
}

#[cfg(test)]
mod unit_tests {
    use crate::db::start_connection;
    use diesel::PgConnection;

    #[tokio::test]
    async fn db_connection_test() -> tide::Result<()> {
        let conn: PgConnection = start_connection().await;
        Ok(())
    }
}

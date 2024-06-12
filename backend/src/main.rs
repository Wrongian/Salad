pub mod funcs;
pub mod models;
pub mod routes;
pub mod schema;
pub mod tests;
use routes::auth::{login, register, logout};
use routes::profile_controller::get_profile;
use tide::new;
use std::env;
pub mod db;
use dotenvy::dotenv;
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use tide_diesel::DieselMiddleware;

// Migration to DB tables creation
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// main function
#[async_std::main]
async fn main() -> tide::Result<()> {
    // load dotenv
    dotenv().expect("No .env file found");

    // setup migrations
    let mut conn = db::start_connection().await;
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    // create app
    let mut app = tide::new();

    // middleware

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

    // Diesel Middleware
    let database_url = env::var("DATABASE_URL").expect("No database url found");
    app.with(DieselMiddleware::new(&database_url).unwrap());

    // set up logging middleware, default log level is 'info'
    femme::start();
    app.with(tide::log::LogMiddleware::new());

    // setup routes

    // auth
    app.at("/login").post(login);
    app.at("/register").post(register);
    app.at("/logout").post(register);
    // profile
    app.at("/profile/:username").get(get_profile);

    // attach to IP and port
    app.listen(funcs::get_url()).await?;

    // return
    Ok(())
}

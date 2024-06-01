pub mod funcs;
pub mod models;
pub mod routes;
pub mod schema;
pub mod tests;
use dotenv::dotenv;
use routes::auth::login;
use routes::auth::register;
use routes::profile_controller::get_profile;
use std::env;
pub mod db;
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};

// Migration to DB tables creation
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
#[async_std::main]
async fn main() -> tide::Result<()> {
    // load dotenv
    dotenv().expect("No .env file found");

    // placeholder test to create a user
    /*
    let new_user = User {
        username: "meme".to_string(),
        password: "meme1".to_string(),
        email: "meme2".to_string(),
        bio: "meme3".to_string(),
        is_private: false,
        salt: "meme4".to_string(),
    };
    */
    log::info!("database url: {}", env::var("DATABASE_URL").unwrap());
    let db_url = env::var("DATABASE_URL").expect("No database url found");

    let mut conn = db::start_connection().await;
    // setup migrations
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    // create app
    let mut app = tide::new();

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
    app.at("/login").post(login);
    app.at("/register").post(register);
    app.at("/profile/:username").get(get_profile);

    // attach to IP and port
    app.listen(funcs::get_url()).await?;

    Ok(())
}

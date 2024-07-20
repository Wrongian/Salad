use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{self as s3};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use http_types::headers::HeaderValue;
use saladify::connectors::buckets::file::setup_buckets;
use saladify::connectors::db::connection::start_connection;
use saladify::helpers::funcs;
use saladify::routes::auth::login::{is_logged_in, login};
use saladify::routes::auth::logout::logout;
use saladify::routes::auth::register::register;
use saladify::routes::follow::create::create_outbound_follow_request;
use saladify::routes::follow::delete::{
    delete_follower, delete_following, delete_outbound_follow_request,
};
use saladify::routes::follow::get::get_follow_status;
use saladify::routes::follow::update::settle_inbound_follow_request;
use saladify::routes::links::create::add_link;
use saladify::routes::links::delete::{delete_link_picture, delete_links};
use saladify::routes::links::get::get_links;
use saladify::routes::links::update::{
    reorder_links, update_link_bio, update_link_href, update_link_picture, update_link_title,
};
use saladify::routes::profiles::get::{get_profile, get_username};
use saladify::routes::profiles::update::{update_display_profile, update_profile_image};
use saladify::types::state::TideState;
use std::env;
use std::sync::Arc;
use tide::security::{CorsMiddleware, Origin};

// Migration to DB tables creation
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// main function
#[tokio::main]
async fn main() -> tide::Result<()> {
    // load dotenv
    dotenv().expect("No .env file found");

    // setup migrations
    let mut conn = start_connection().await;
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
        tempdir: tempfile::tempdir()?,
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
    app.at("/logout").get(logout);
    app.at("/logged-in").get(is_logged_in);

    // profile
    app.at("/profiles/:username").get(get_profile);
    app.at("/profiles/display").put(update_display_profile);
    app.at("/profiles/image/:ext").put(update_profile_image);

    // links
    app.at("/links/:username").get(get_links);
    app.at("/links").post(add_link);
    app.at("/links/reorder").post(reorder_links);
    app.at("/links/title/:link_id").put(update_link_title);
    app.at("/links/bio/:link_id").put(update_link_bio);
    app.at("/links/href/:link_id").put(update_link_href);
    app.at("/links/:link_id/image/:ext")
        .put(update_link_picture);
    app.at("/links/:link_id/image").delete(delete_link_picture);
    app.at("/links/:link_id").delete(delete_links);

    // follow
    app.at("/follow").put(settle_inbound_follow_request);
    app.at("/follower").delete(delete_follower);
    app.at("/following").delete(delete_following);
    app.at("/follow-status").get(get_follow_status);
    app.at("/follow-request")
        .post(create_outbound_follow_request)
        .delete(delete_outbound_follow_request);

    // misc
    app.at("get-username").get(get_username);

    // attach to IP and port
    app.listen(funcs::get_url()).await?;

    // return
    Ok(())
}

#[cfg(test)]
mod unit_tests {
    use saladify::connectors::db::connection::start_connection;

    #[tokio::test]
    async fn it_can_connect_to_db() -> tide::Result<()> {
        start_connection().await;
        Ok(())
    }
}

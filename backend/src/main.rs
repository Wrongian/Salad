pub mod models;
pub mod routes;
pub mod schema;
pub mod tests;
pub mod funcs;
use dotenv::dotenv;
use routes::auth::login;
use routes::auth::register;
use routes::profile_controller::get_profile;
use std::env;
pub mod db;
use http_types::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};


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

    // create app
    let mut app = tide::new();

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS, PUT".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from(vec!["http://localhost:5173"]))
        .allow_credentials(false);

    app.with(cors);

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

pub mod routes;
pub mod schema;
pub mod models;
use std::env;
use routes::auth::login;
use routes::auth::register;
use dotenv::dotenv;
pub mod db;

#[async_std::main]
async fn main() -> tide::Result<()>{
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

    // setup routes
    app.at("/login").post(login);
    app.at("/register").post(register);

    // attach to IP and port
    let ip_address = env::var("IP_ADDRESS").expect("Ip address needs to be set in .env");
    let port = env::var("PORT").expect("Port needs to be set in .env");
    let ip_port = ip_address.clone() + ":" + &port;
    app.listen(ip_port).await?;

    Ok(())
}

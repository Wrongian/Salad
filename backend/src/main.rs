pub mod routes;
use std::env;

use routes::auth::login;
use routes::auth::register;
use dotenv::dotenv;
use postgres::{Client, NoTls, Error};

#[async_std::main]
async fn main() -> tide::Result<()>{
    // load dotenv
    dotenv().expect("No .env file found");
    
    // setup database
    let db_host = env::var("POSTGRES_HOST").expect("");
    let db_port = env::var("POSTGRES_PORT").expect("");
    let db_user = env::var("POSTGRES_USER").expect("");
    let db_password = env::var("POSTGRES_PASSWORD").expect("");
    let db_name = env::var("POSTGRES_NAME").expect("");

    let database_url = db_host.clone() + "://" + &db_user + ":" + &db_password + "@localhost:" + &db_port + "/" + &db_name;
    let client = Client::connect(&database_url,NoTls)?;

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

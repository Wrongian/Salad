pub mod routes;
use std::env;

use routes::auth::login;
use routes::auth::register;
use dotenv::dotenv;

#[async_std::main]
async fn main() -> tide::Result<()>{
    // load dotenv
    dotenv().expect("No .env file found");

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

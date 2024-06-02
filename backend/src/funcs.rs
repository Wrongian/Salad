use dotenvy::dotenv;
use std::env;

// gets the server url
pub fn get_url() -> String {
    dotenv().expect("No .env file found");

    let ip_address = env::var("IP_ADDRESS").expect("Ip address needs to be set in .env");
    let port = env::var("BACKEND_PORT").expect("Port needs to be set in .env");
    let ip_port = ip_address.clone() + ":" + &port;
    ip_port
}

use crate::types::error::Error;
use chrono::{NaiveDateTime, TimeDelta};
use dotenvy::dotenv;
use std::env;

// various miscellaneous functions

// gets the server url
pub fn get_url() -> String {
    dotenv().expect("No .env file found");

    let ip_address = env::var("IP_ADDRESS").expect("Ip address needs to be set in .env");
    let port = env::var("BACKEND_PORT").expect("Port needs to be set in .env");
    let ip_port = ip_address.clone() + ":" + &port;
    ip_port
}

// takes one chrono timestamp and a time interval in chrono duration and returns whether its expired
pub fn is_expired(time_before: NaiveDateTime, expiry_duration: TimeDelta) -> Result<bool, Error> {
    let now_secs = chrono::Local::now().timestamp();
    let expiry_time = match (time_before + expiry_duration)
        .and_local_timezone(chrono::Local)
        .single()
    {
        Some(secs) => secs.timestamp(),
        None => return Err(Error::DatetimeError()),
    };
    // print!("now:{}\n", now_secs);
    // print!("expiry{}", expiry_time);
    if now_secs > expiry_time {
        return Ok(true);
    }
    return Ok(false);
}

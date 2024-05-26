use tide::Request;
use tide::Response;
use validator::{Validate, ValidationError};

use crate::db::start_connection;
use crate::db::user::get_user_profile_by_username;

#[derive(Debug, serde::Deserialize, Validate)]
pub struct GetProfileParams {
    #[validate(length(min = 5, max = 50))]
    username: String,
}

#[derive(Debug, serde::Serialize)]
struct GetProfileResponseBody {
    display_name: String,
    bio: String,
    username: String,
    picture: String,
    following: i32,
    followers: i32,
}

#[derive(Debug, serde::Deserialize, Validate)]
struct UpdateProfileBody {
    display_name: String,
    bio: String,
    picture: String,
}

fn build_response(body: impl serde::Serialize, status: u16) -> tide::Result {
    // build response
    let response = Response::builder(status)
        .body(tide::Body::from_json(&body)?)
        .build();
    Ok(response)
}

fn build_error(message: String, status: u16) -> tide::Result {
    let response = Response::builder(status).body(message).build();
    Ok(response)
}

pub async fn update_profile(req: Request<()>) -> tide::Result {
    // TODO: implementation
    Ok(Response::builder(200).build())
}

pub async fn get_profile(req: Request<()>) -> tide::Result {
    // let username = req.query::<GetProfileParams>()?.username;
    let username = match req.param("username") {
        Ok(name) => name.to_owned(),
        Err(e) => return build_error(e.to_string(), 400),
    };

    log::info!("Obtained username in get_profile: {}", &username);

    let mut conn = start_connection().await;

    let profile_query_result = get_user_profile_by_username(&mut conn, &username).await;

    let res_body = match profile_query_result {
        Ok(profile) => GetProfileResponseBody {
            display_name: profile.display_name,
            bio: profile.bio.unwrap_or("".to_owned()),
            username: profile.username,
            picture: String::from("Placeholder for picture"),
            followers: 0,
            following: 0,
        },
        Err(message) => {
            log::error!("{}", message);
            return build_error(message, 500);
        }
    };

    build_response(res_body, 200)
}

pub async fn delete_profile(req: Request<()>) -> tide::Result {
    // TODO: implementation
    Ok(Response::builder(200).build())
}

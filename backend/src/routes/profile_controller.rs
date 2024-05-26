use serde::Deserialize;
use serde::Serialize;
use tide::Request;
use tide::Response;
use validator::{Validate, ValidationError};

use crate::db::start_connection;
use crate::db::user::get_user_profile_by_username;

#[derive(Debug, Deserialize, Validate)]
pub struct GetProfileParams {
    #[validate(length(min = 5, max = 50))]
    username: String,
}

#[derive(Debug, Serialize)]
struct GetProfileResponseBody {
    display_name: String,
    bio: String,
    username: String,
    picture: String,
    following: i32,
    followers: i32,
}

#[derive(Debug, Deserialize, Validate)]
struct UpdateProfileBody {
    display_name: String,
    bio: String,
    picture: String,
}

fn build_response(body: &impl Serialize, status: u16) -> tide::Result {
    // build response
    let response = Response::builder(status)
        .body(tide::Body::from_json(&body)?)
        .build();
    Ok(response)
}

pub async fn update_profile(mut req: Request<()>) -> tide::Result {
    // TODO: implementation
    Ok(Response::builder(200).build())
}

pub async fn get_profile(mut req: Request<()>) -> tide::Result {
    let params = GetProfileParams {
        username: req.param("username").unwrap_or("").to_owned(),
    };

    let mut conn = start_connection().await;

    let profile = get_user_profile_by_username(&mut conn, &params.username).await;
    let res_body = GetProfileResponseBody {
        display_name: profile.display_name,
        bio: profile.bio.unwrap_or("".to_owned()),
        username: profile.username,
        picture: String::from("Placeholder for picture"),
        followers: 0,
        following: 0,
    };

    build_response(&res_body, 200)
}

pub async fn delete_profile(mut req: Request<()>) -> tide::Result {
    // TODO: implementation
    Ok(Response::builder(200).build())
}

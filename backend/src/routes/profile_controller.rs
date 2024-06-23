use std::borrow::Borrow;
use std::sync::Arc;

use crate::db::start_connection;
use crate::db::user::get_user_profile_by_username;
use crate::TideState;
use tide::Request;
use tide::Response;
use validator::Validate;

// Profile parameters struct
#[derive(Debug, serde::Deserialize, Validate)]
pub struct GetProfileParams {
    #[validate(length(min = 5, max = 50))]
    username: String,
}

// Profile parameters for getting the profile response body
#[derive(Debug, serde::Serialize)]
struct GetProfileResponseBody {
    display_name: String,
    bio: String,
    is_owner: bool,
    picture: String,
    following: Option<i32>,
    followers: Option<i32>,
}

// Parameters for the route to update the profile response body
#[derive(Debug, serde::Deserialize, Validate)]
struct UpdateProfileBody {
    display_name: String,
    bio: String,
    picture: String,
}

// build the standard response
fn build_response(body: impl serde::Serialize, status: u16) -> tide::Result {
    // build response
    let response = Response::builder(status)
        .body(tide::Body::from_json(&body)?)
        .build();
    Ok(response)
}

// build an error response
fn build_error(message: String, status: u16) -> tide::Result {
    let response = Response::builder(status).body(message).build();
    Ok(response)
}

// update profile response body
pub async fn update_profile(req: Request<Arc<TideState>>) -> tide::Result {
    // TODO: implementation
    Ok(Response::builder(200).build())
}

// Get profile route
pub async fn get_profile(req: Request<Arc<TideState>>) -> tide::Result {
    let username = match req.param("username") {
        Ok(name) => name.to_owned(),
        // last match clause should not happen.
        Err(e) => return build_error(e.to_string(), 400),
    };
    // let ses = req.session().clone().validate();
    // println!("{}",ses.is_some());
    // println!("{}", req.session().id());
    // println!("{}", req.session().is_destroyed());
    // println!("{}", req.session().is_expired());
    // get relevant username session field
    let session_username: String = req.session().get("username").unwrap_or("".to_owned());
    println!("session username: {}", &session_username);
    log::info!("Obtained username in get_profile: {}", &username);

    let state = req.state();
    let mut conn = start_connection().await;

    // get profile view from database
    let profile_query_result = get_user_profile_by_username(&mut conn, &username).await;
    let is_owner = session_username == username;

    let res_body = match profile_query_result {
        Ok(profile) => {
            // TODO: we need to update is_private to profile.is_private when DB is updated
            let is_private = true;
            if !is_owner && is_private {
                // return object with certain fields defaulted to empty values
                GetProfileResponseBody {
                    display_name: profile.display_name,
                    bio: "".to_owned(),
                    picture: String::from("picture placeholder"),
                    is_owner: false,
                    followers: None,
                    following: None,
                }
            } else {
                // either is_owner or not private account, either ways all fields are accessible.
                // So return all fields.

                // query for profile picture and add it to response body
                // handling of cases
                // let byte_stream = get_profile_image(&state.s3_client, profile.id.to_string()).await;
                // if byte_stream.is_err() {
                //     log::error!("{}", "An error occurred in retrieving profile image.");
                //     return build_error(
                //         "An error occurred in retrieving profile image.".to_string(),
                //         400,
                //     );
                // }
                // let maybe_picture = collect_as_bytes(byte_stream.unwrap()).await;

                // if maybe_picture.is_err() {
                //     return build_error(
                //         "An error occurred in streaming profile image.".to_string(),
                //         400,
                //     );
                // }

                // let pic_string = String::from_utf8(maybe_picture.unwrap());
                // if pic_string.is_err() {
                //     return build_error(
                //         "An unexpected error occurred in streaming profile images".to_string(),
                //         500,
                //     );
                // }

                GetProfileResponseBody {
                    display_name: profile.display_name,
                    bio: profile.bio.unwrap_or("".to_owned()),
                    is_owner,
                    // picture: pic_string.unwrap(),
                    picture: String::from("placeholder"),
                    followers: Some(0),
                    following: Some(0),
                }
            }
        }
        Err(message) => {
            log::error!("{}", message);
            return build_error(message, 500);
        }
    };

    build_response(res_body, 200)
}

pub async fn get_links(req: Request<Arc<TideState>>) -> tide::Result {
    Ok(Response::builder(200).build())
}
// delete profile response builder
pub async fn delete_profile(req: Request<Arc<TideState>>) -> tide::Result {
    // TODO: implementation
    Ok(Response::builder(200).build())
}

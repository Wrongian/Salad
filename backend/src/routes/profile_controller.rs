use std::env;
use std::sync::Arc;

use crate::buckets::file::{delete_s3_profile_image, update_s3_profile_image};
use crate::db::image::{create_profile_image, get_profile_image, delete_profile_image};
use crate::db::user::{get_user_profile_by_username, update_user_by_id};
use crate::helpers::auth::{get_session_user_id, get_session_username};
use crate::helpers::response::{build_error, build_response, build_standard_response};
use crate::models::images::InsertProfileImage;
use crate::models::users::UpdateUser;
use crate::TideState;
use aws_sdk_s3::primitives::ByteStream;
use tide::log::{error, info, warn};
use tide::Request;
use tide::Response;
use uuid::Uuid;
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Validate)]
struct UpdateDisplayProfilePayload {
    display_name: Option<String>,
    bio: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct UploadProfileImageResponseBody {
    href: String,
    result: bool,
    err: String
}

#[derive(Debug, serde::Serialize)]
struct UsernameResponseBody {
    username: String,
}
// update profile response body
pub async fn update_display_profile(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(_) => return build_error("invalid session!".to_string(), 400),
    };
    // get json body as UpdateProfilePayload
    let update_body: UpdateDisplayProfilePayload = match req.body_json().await {
        Ok(body) => body,
        Err(_) => return build_error("Bad request body.".to_string(), 400),
    };

    // construct UpdateProfile model
    let update_user = UpdateUser {
        username: None,
        password: None,
        salt: None,
        email: None,
        is_private: None,
        bio: update_body.bio,
        display_name: update_body.display_name,
    };
    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    // call orm
    return match update_user_by_id(&mut conn, user_id, &update_user).await {
        Ok(result) => build_standard_response(result, "".to_string(), 200),
        Err(err) => build_error(err, 400),
    };
}

pub async fn update_profile_image(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user_id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(err) => return build_error("invalid session!".to_string(), 400),
    };

    // get :name from params
    let image_name = match req.param("ext") {
        Ok(ext) => [Uuid::new_v4().to_string(), ext.to_string()].join("."),
        Err(err) => return Err(err),
    };

    // process the req body as bytes
    let bytes = req.body_bytes().await;

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    let s3_client = &state.s3_client;

    // remove previous file; if any
    match get_profile_image(&mut conn, user_id).await {
        Ok(img) => {
            // remove from s3 if present
            let result = delete_s3_profile_image(s3_client, img.filename).await;
            let db_result = delete_profile_image(&mut conn, user_id).await;
            if result.is_err() {
                error!("Error deleting profile image: {}", result.unwrap_err());
            }

            if db_result.is_err() {
                error!("Error deleting profile image from db.");
            }
        }
        // do nothing if not found
        Err(msg) => (),
    }

    // get uploaded file as bytes
    if bytes.is_err() {
        error!("failed to get bytes from request body.");
        return build_error("Failed to get bytes from body.".to_string(), 400);
    }

    // upload file to s3
    match update_s3_profile_image(
        s3_client,
        image_name.clone(),
        ByteStream::from(bytes.unwrap()),
    )
    .await
    {
        Ok(()) => (),
        Err(msg) => {
            error!("upload to s3 failed with error: {}", msg);
            return build_error("Failed to upload profile image".to_string(), 400);
        }
    }
    // create cdn href
    let profile_cdn_origin = env::var("PROFILE_IMAGE_CDN").expect("unable to process cdn");
    let cdn_href = [profile_cdn_origin, image_name.clone()].join("/");
    // create src href in db
    let payload = InsertProfileImage {
        img_src: cdn_href.clone(),
        filename: image_name.clone(),
        user_id,
    };
    info!("creating cdn href.. {}", cdn_href.clone());

    match create_profile_image(&mut conn, &payload).await {
        Ok(img) => build_response(UploadProfileImageResponseBody { href: cdn_href, result: true, err: "".to_string()}, 200),
        Err(msg) => build_error(msg, 400),
    }
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
    info!("session username: {}", &session_username);
    info!("Obtained username in get_profile: {}", &username);

    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    // get profile view from database
    let profile_query_result = get_user_profile_by_username(&mut conn, &username).await;
    let is_owner = session_username == username;

    let res_body = match profile_query_result {
        Ok(profile) => {
            let is_private = profile.is_private;
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

                // get cdn_href from db
                let picture = get_profile_image(&mut conn, profile.id)
                    .await
                    .map(|img| img.img_src)
                    .unwrap_or_else(|e| {
                        warn!(
                            "Error in retrieving profile picture, using default. (error: {})",
                            e
                        );
                        String::from("")
                    });

                GetProfileResponseBody {
                    display_name: profile.display_name,
                    bio: profile.bio.unwrap_or("".to_owned()),
                    is_owner,
                    picture,
                    followers: Some(0),
                    following: Some(0),
                }
            }
        }
        Err(message) => {
            error!("error in retrieving profile: {}", message);
            return build_error(message, 500);
        }
    };

    build_response(res_body, 200)
}

// delete profile response builder
pub async fn delete_profile(req: Request<Arc<TideState>>) -> tide::Result {
    // TODO: implementation
    Ok(Response::builder(200).build())
}

// Gets the session username
pub async fn get_username(req: Request<Arc<TideState>>) -> tide::Result {
    // get session username from session
    let session_username = match get_session_username(&req) {
        Ok(session_usr) => session_usr,
        Err(_) => return build_error("invalid session!".to_string(), 400),
    };
    return build_response( UsernameResponseBody{username : session_username}, 200)
}




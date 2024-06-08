use crate::db::start_connection;
use crate::db::user::get_user_profile_by_username;
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
pub async fn update_profile(req: Request<()>) -> tide::Result {
    // TODO: implementation
    Ok(Response::builder(200).build())
}

// Get profile route
pub async fn get_profile(req: Request<()>) -> tide::Result {
    let username = match req.param("username") {
        Ok(name) => name.to_owned(),
        // last match clause should not happen.
        Err(e) => return build_error(e.to_string(), 400),
    };
    // get relevant username session field
    let session_username: String = req.session().get("username").unwrap_or("".to_owned());
    println!("session username: {}", &session_username);
    log::info!("Obtained username in get_profile: {}", &username);

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
                GetProfileResponseBody {
                    display_name: profile.display_name,
                    bio: profile.bio.unwrap_or("".to_owned()),
                    picture: String::from("picture placeholder"),
                    is_owner,
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

// delete profile response builder
pub async fn delete_profile(req: Request<()>) -> tide::Result {
    // TODO: implementation
    Ok(Response::builder(200).build())
}

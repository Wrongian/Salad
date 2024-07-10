use std::sync::Arc;

use tide::{
    log::{error, info, warn},
    Request,
};
use validator::Validate;

use crate::{
    connectors::db::{
        image::get_profile_image,
        user::{check_username_present, get_user_profile_by_username},
    },
    helpers::{auth::get_session_username, params::extract_username_from_params},
    types::{error::Error, response::Response, state::TideState},
};

// Profile parameters struct
#[derive(Debug, serde::Deserialize, Validate)]
pub struct GetProfileParams {
    #[validate(length(min = 5, max = 50))]
    username: String,
}

#[derive(Debug, serde::Serialize)]
struct UsernameResponseBody {
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

// Gets the session username
pub async fn get_username(req: Request<Arc<TideState>>) -> tide::Result {
    // get session username from session
    let session_username = match get_session_username(&req) {
        Ok(session_usr) => session_usr,
        Err(e) => return e.into_response(),
    };
    return Response::new(UsernameResponseBody {
        username: session_username,
    })
    .into_response();
}

// Get profile route
pub async fn get_profile(req: Request<Arc<TideState>>) -> tide::Result {
    let username = match extract_username_from_params(&req) {
        Ok(name) => name.to_owned(),
        Err(e) => return e.into_response(),
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

    // check username exists
    if !check_username_present(&mut conn, &username).await {
        return Error::NotFoundError(String::from("User")).into_response();
    }

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
        Err(e) => {
            error!("error in retrieving profile: {}", e);
            return Error::DieselError(e).into_response();
        }
    };
    Response::new(res_body).into_response()
}

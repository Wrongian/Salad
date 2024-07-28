use std::{env, sync::Arc};

use aws_sdk_s3::primitives::ByteStream;
use tide::{
    log::{error, info},
    Request,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    connectors::{
        buckets::file::{delete_s3_profile_image, update_s3_profile_image},
        db::{
            image::{create_profile_image, delete_profile_image, get_profile_image},
            user::update_user_by_id,
        },
    },
    helpers::auth::get_session_user_id,
    models::{images::InsertProfileImage, users::UpdateUser},
    types::{
        error::{Error, RequestErrors, S3Errors},
        response::Response,
        state::TideState,
    },
};

#[derive(Debug, serde::Deserialize, serde::Serialize, Validate)]
struct UpdateDisplayProfilePayload {
    display_name: Option<String>,
    bio: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct UploadProfileImageResponseBody {
    href: String,
}

// update profile response body
pub async fn update_display_profile(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };
    // get json body as UpdateProfilePayload
    let update_body: UpdateDisplayProfilePayload = match req.body_json().await {
        Ok(body) => body,
        _ => return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response(),
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
        Ok(_result) => Response::empty().into_response(),
        Err(err) => Error::DieselError(err).into_response(),
    };
}

pub async fn update_profile_image(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user_id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
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
        Err(_msg) => (),
    }

    // get uploaded file as bytes
    if bytes.is_err() {
        error!("failed to get bytes from request body.");
        return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response();
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
            return Error::S3Error(S3Errors::FailedToUploadImage).into_response();
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
        Ok(_img) => {
            Response::new(UploadProfileImageResponseBody { href: cdn_href }).into_response()
        }
        Err(e) => Error::DieselError(e).into_response(),
    }
}

use crate::{
    buckets::file::{delete_s3_link_image, update_s3_link_image},
    db::{
        image::{create_link_image, get_link_image, update_link_image},
        link::{get_link_by_id, get_user_link_by_id, update_link_by_id},
        DBConnection,
    },
    helpers::{
        auth::get_session_user_id,
        response::{build_error, build_response, build_standard_response},
    },
    models::{
        images::{GetImage, InsertLinkImage, UpdateImage},
        links::UpdateLink,
    },
    TideState,
};
use aws_sdk_s3::primitives::ByteStream;
use diesel::PgConnection;
use log::warn;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, env, sync::Arc};
use tide::{
    log::{error, info},
    Request,
};
use tokio::fs::OpenOptions;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Deserialize, Validate, Serialize)]
struct CreateLinkParams {
    title: Option<String>,
    bio: Option<String>,
    href: String,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
struct UpdateTitlePayload {
    title: String,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
struct UpdateBioPayload {
    bio: String,
}

#[derive(Debug, Deserialize, Validate, Serialize)]
struct UpdateHrefPayload {
    href: String,
}

#[derive(Debug, Serialize)]
struct UploadLinkResponseBody {
    href: String,
}

async fn handle_validation_errors(e: ValidationErrors) -> tide::Result {
    let mut error_string: String = "".to_string();
    let validations = e.field_errors();
    let values = validations.values();
    for validation_errors in values {
        for validation_error in validation_errors.iter() {
            let error_message = validation_error.message.borrow();
            match error_message {
                Some(message) => {
                    error_string += message.borrow();
                    error_string += ".";
                }
                None => {}
            }
        }
    }

    return build_standard_response(false, error_string, 400);
}
// POST end point for adding a link
pub async fn add_link(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get payload
    let link_params: CreateLinkParams;
    match req.body_json().await {
        Ok(params) => {
            link_params = params;
        }
        Err(e) => {
            return build_standard_response(false, "Bad Request Body".to_string(), 400);
        }
    }

    // validate payload
    match link_params.validate() {
        Err(e) => return handle_validation_errors(e).await,
        _ => (),
    }

    // add to database
    let state = req.state();
    let mut conn: DBConnection = state.tide_pool.get().unwrap();

    // return 200; otherwise 400
    build_standard_response(true, "".to_string(), 200)
}

// TODO: combine update link title, bio & href into the same endpoint
pub async fn update_link_title(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // extract link id
    let link_id = match req.param("link_id").and_then(|id| {
        id.parse::<i32>()
            .map_err(|_| tide::Error::from_str(400, "Invalid link_id provided."))
    }) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // extract title payload body
    let update_title: UpdateTitlePayload = match req.body_json().await {
        Ok(title_obj) => title_obj,
        Err(message) => return build_error("Bad request body.".to_string(), 400),
    };

    // validate title
    match update_title.validate() {
        Err(e) => return handle_validation_errors(e).await,
        _ => (),
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    // check user link with link_id exists
    match get_user_link_by_id(&mut conn, link_id, user_id).await {
        Ok(res) => (),
        Err(_) => return build_error("Link does not exist.".to_string(), 400),
    };

    // update the link
    let update_link = UpdateLink {
        user_id: None,
        next_id: None,
        prev_id: None,
        description: None,
        title: Some(update_title.title),
        href: None,
    };

    let result = match update_link_by_id(&mut conn, &update_link, link_id).await {
        Ok(result) => result,
        Err(message) => return build_error("Failed to update the provided link.".to_string(), 400),
    };

    build_standard_response(result, "".to_string(), 200)
}

pub async fn update_link_bio(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // extract link id
    let link_id = match req.param("link_id").and_then(|id| {
        id.parse::<i32>()
            .map_err(|_| tide::Error::from_str(400, "Invalid link_id provided."))
    }) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // extract title payload body
    let update_bio: UpdateBioPayload = match req.body_json().await {
        Ok(title_obj) => title_obj,
        Err(message) => return build_error("Bad request body.".to_string(), 400),
    };

    // validate title
    match update_bio.validate() {
        Err(e) => return handle_validation_errors(e).await,
        _ => (),
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    // check user link with link_id exists
    match get_user_link_by_id(&mut conn, link_id, user_id).await {
        Ok(res) => (),
        Err(_) => return build_error("Link does not exist.".to_string(), 400),
    };

    // update the link
    let update_bio = UpdateLink {
        user_id: None,
        next_id: None,
        prev_id: None,
        title: None,
        description: Some(update_bio.bio),
        href: None,
    };

    let result = match update_link_by_id(&mut conn, &update_bio, link_id).await {
        Ok(result) => result,
        Err(message) => return build_error("Failed to update the provided link.".to_string(), 400),
    };

    build_standard_response(result, "".to_string(), 200)
}

pub async fn update_link_href(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // extract link id
    let link_id = match req.param("link_id").and_then(|id| {
        id.parse::<i32>()
            .map_err(|_| tide::Error::from_str(400, "Invalid link_id provided."))
    }) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // extract title payload body
    let updated_href: UpdateHrefPayload = match req.body_json().await {
        Ok(title_obj) => title_obj,
        Err(message) => return build_error("Bad request body.".to_string(), 400),
    };

    // validate title
    match updated_href.validate() {
        Err(e) => return handle_validation_errors(e).await,
        _ => (),
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    // check user link with link_id exists
    match get_user_link_by_id(&mut conn, link_id, user_id).await {
        Ok(res) => (),
        Err(_) => return build_error("Link does not exist.".to_string(), 400),
    };
    // update the link
    let update_link = UpdateLink {
        user_id: None,
        next_id: None,
        prev_id: None,
        description: None,
        title: None,
        href: Some(updated_href.href),
    };

    let result = match update_link_by_id(&mut conn, &update_link, link_id).await {
        Ok(result) => result,
        Err(message) => return build_error("Failed to update the provided link.".to_string(), 400),
    };

    build_standard_response(result, "".to_string(), 200)
}

pub async fn update_link_picture(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(err) => return build_error("invalid session!".to_string(), 400),
    };

    // get user link from link id from params
    let link_id = match req.param("link_id").and_then(|id| {
        id.parse::<i32>()
            .map_err(|_| tide::Error::from_str(400, "Invalid link_id provided."))
    }) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // get :name from params
    let image_name = match req.param("name") {
        Ok(name) => name.to_string(),
        Err(err) => return Err(err),
    };

    // process the req body as bytes
    let bytes = req.body_bytes().await;

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    let s3_client = &state.s3_client;

    // assert link_id belongs to user_id
    match get_user_link_by_id(&mut conn, link_id, user_id).await {
        Ok(link) => (),
        Err(msg) => return build_error("Invalid link provided.".to_string(), 400),
    }

    // remove previous file; if any
    match get_link_image(&mut conn, link_id).await {
        Ok(img) => {
            // remove from s3 if present
            let result = delete_s3_link_image(s3_client, img.filename).await;
            if result.is_err() {
                error!("Error deleting link image: {}", result.unwrap_err());
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
    match update_s3_link_image(
        s3_client,
        image_name.clone(),
        ByteStream::from(bytes.unwrap()),
    )
    .await
    {
        Ok(()) => (),
        Err(msg) => {
            error!("upload to s3 failed with error: {}", msg);
            return build_error("Failed to upload link image".to_string(), 400);
        }
    }
    // create cdn href
    let link_cdn_origin = env::var("LINK_IMAGE_CDN").expect("unable to process cdn");
    let cdn_href = [link_cdn_origin, image_name.clone()].join("/");
    // create src href in db
    let payload = InsertLinkImage {
        img_src: cdn_href.clone(),
        filename: image_name.clone(),
        link_id,
    };
    info!("creating cdn href.. {}", cdn_href.clone());

    match create_link_image(&mut conn, &payload).await {
        Ok(img) => build_response(UploadLinkResponseBody { href: cdn_href }, 200),
        Err(msg) => build_error(msg, 400),
    }
}

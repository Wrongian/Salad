use crate::{
    buckets::file::{delete_s3_link_image, update_s3_link_image},
    db::{
        self,
        image::{create_link_image, delete_link_image, get_link_image, update_link_image},
        link::{
            delete_link_by_id, get_link_by_id, get_user_link_by_id, get_user_links_by_id,
            reorder_link, update_link_by_id,
        },
        user::get_user_profile_by_username,
        DBConnection,
    },
    helpers::{
        auth::{get_session_user_id, get_session_username},
        links::linearise,
        response::{build_error, build_response, build_standard_response},
    },
    models::{
        images::{GetImage, InsertLinkImage, UpdateImage},
        links::{GetLink, InsertLink, UpdateLink},
    },
    TideState,
};
use aws_sdk_s3::primitives::ByteStream;
use log::warn;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, env, sync::Arc};
use tide::{
    log::{error, info},
    Request,
};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Deserialize, Validate, Serialize)]
struct CreateLinkParams {
    title: Option<String>,
    bio: Option<String>,
    href: String,
    parent_id: Option<i32>,
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

#[derive(Debug, Serialize)]
struct GetLinksResponseBody {
    links: Vec<GetLink>,
}

#[derive(Debug, Deserialize)]
struct ReorderLinksPayload {
    link_id: i32,
    new_position_id: i32,
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
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };
    // get payload
    let link_params: CreateLinkParams;
    match req.body_json().await {
        Ok(params) => {
            link_params = params;
        }
        Err(e) => {
            return build_error("Bad Request Body".to_string(), 400);
        }
    }

    // validate payload
    match link_params.validate() {
        Err(e) => return handle_validation_errors(e).await,
        _ => (),
    }

    let state = req.state();
    let mut conn: DBConnection = state.tide_pool.get().unwrap();

    // check if parent_id belongs to user_id
    if link_params.parent_id.is_some() {
        match get_user_link_by_id(&mut conn, link_params.parent_id.unwrap(), user_id).await {
            Ok(_) => (),
            Err(_) => return build_error("Invalid parent id provided.".to_string(), 400),
        }
    }

    // add to database
    let insert_link = InsertLink {
        user_id,
        next_id: link_params.parent_id,
        description: link_params.bio,
        title: link_params.title,
        href: link_params.href,
    };

    match db::link::create(&mut conn, &insert_link).await {
        Ok(_) => build_standard_response(true, "".to_string(), 200),
        Err(e) => {
            error!("Error creating link. {:?}, Error: {}", insert_link, e);
            return build_error("Error occurred while creating link.".to_string(), 400);
        }
    }
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
        Err(err) => return Err(err),
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

pub async fn delete_link_picture(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // get user link from link id from params
    let link_id = match req.param("link_id").and_then(|id| {
        id.parse::<i32>().map_err(|e| {
            error!("Error in parsing link_id: {} {:?}", id, e);
            tide::Error::from_str(400, "Invalid link_id provided.")
        })
    }) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    let s3_client = &state.s3_client;

    // assert link_id belongs to user_id
    match get_user_link_by_id(&mut conn, link_id, user_id).await {
        Ok(link) => (),
        Err(msg) => return build_error("Invalid link provided.".to_string(), 400),
    };

    // get image by id
    let img_obj = match get_link_image(&mut conn, link_id).await {
        Ok(img) => img,
        Err(msg) => return build_error("Image not found.".to_string(), 400),
    };

    // delete image from db
    match delete_link_by_id(&mut conn, link_id).await {
        Ok(_) => (),
        Err(msg) => {
            warn!("Failed to delete link: {}", msg);
            return build_error("Error occurred in deleting link image.".to_string(), 400);
        }
    };

    // delete image from s3
    match delete_s3_link_image(s3_client, img_obj.filename).await {
        Ok(res) => build_standard_response(true, "".to_string(), 200),
        Err(msg) => {
            error!("Unable to delete link from s3: {}", msg);
            return build_error("Error occurred in deleting link image.".to_string(), 400);
        }
    }
}

pub async fn get_links(req: Request<Arc<TideState>>) -> tide::Result {
    // get session username from session
    let session_username = match get_session_username(&req) {
        Ok(session_usr) => session_usr,
        Err(err) => return Err(err),
    };

    // get username from params
    let username = match req.param("username") {
        Ok(username) => username.to_string(),
        Err(err) => return Err(err),
    };

    let is_owner = session_username == username;

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    // check if profile is private, if it is then return empty vec
    let profile = match get_user_profile_by_username(&mut conn, &username).await {
        Ok(data) => data,
        Err(e) => return build_error("Error in verifying profile".to_string(), 400),
    };

    if !is_owner && profile.is_private {
        // if origin is not owner and querying a private profile, return empty links
        return build_response(GetLinksResponseBody { links: Vec::new() }, 200);
    }
    // otherwise either owner or querying a public profile.
    // Thus, get all links and return
    match get_user_links_by_id(&mut conn, profile.id).await {
        Ok(links) => build_response(
            GetLinksResponseBody {
                links: linearise(&links),
            },
            200,
        ),
        Err(msg) => {
            error!("Error in retrieving user links by id: {}", msg);
            build_error("Error in getting links".to_string(), 400)
        }
    }
}

pub async fn delete_links(req: Request<Arc<TideState>>) -> tide::Result {
    // get user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // get link id from params
    let link_id = match req.param("link_id").and_then(|id| {
        id.parse::<i32>()
            .map_err(|_| tide::Error::from_str(400, "Invalid link_id provided."))
    }) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    // assert link_id belongs to user_id
    match get_user_link_by_id(&mut conn, link_id, user_id).await {
        Ok(_) => (),
        Err(_) => return build_error("Invalid link provided.".to_string(), 400),
    }

    // delete image for link
    match get_link_image(&mut conn, link_id).await {
        Ok(link) => {
            let delete_result = delete_s3_link_image(&state.s3_client, link.filename).await;
            let delete_db_result = delete_link_image(&mut conn, link_id).await;
            if delete_result.is_err() {
                error!("Error in deleting image from s3: {:?}", delete_result.err());
            }
            if delete_db_result.is_err() {
                error!(
                    "Error in deleting image from db: {:?}",
                    delete_db_result.err()
                );
            }
        }
        Err(_) => (),
    }

    // delete link_id
    match delete_link_by_id(&mut conn, link_id).await {
        Ok(res) => build_standard_response(res, "".to_string(), 200),
        Err(msg) => {
            error!("Error in deleting link: {}", msg);
            return build_error("Error occurred in deleting link.".to_string(), 400);
        }
    }
}

pub async fn reorder_links(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(err) => return Err(err),
    };

    // get reordering links
    let reorder_link_params: ReorderLinksPayload = match req.body_json().await {
        Ok(body) => body,
        Err(e) => return Err(e),
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    // assert both links belong to user_id
    match get_user_link_by_id(&mut conn, reorder_link_params.link_id, user_id).await {
        Ok(_) => (),
        Err(_) => return build_error("Invalid link_id provided.".to_string(), 400),
    };

    match get_user_link_by_id(&mut conn, reorder_link_params.new_position_id, user_id).await {
        Ok(_) => (),
        Err(_) => return build_error("Invalid new_position_id provided.".to_string(), 400),
    };

    // reorder links
    match reorder_link(
        &mut conn,
        reorder_link_params.link_id,
        reorder_link_params.new_position_id,
    )
    .await
    {
        Ok(_) => build_standard_response(true, "".to_string(), 200),
        Err(e) => {
            error!("Error in reordering link: {}", e);
            build_error("Error occurred in reordering link.".to_string(), 400)
        }
    }
}

use crate::{
    db::{
        link::{get_link_by_id, get_user_link_by_id, update_link_by_id},
        DBConnection,
    },
    helpers::{
        auth::get_session_user_id,
        response::{build_error, build_response, build_standard_response},
    },
    models::links::UpdateLink,
    TideState,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, sync::Arc};
use tide::Request;
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
    // remove previous file; if any

    // get uploaded file

    // upload file to s3

    // create cdn href

    // update src href in db
    build_response(
        UploadLinkResponseBody {
            href: "".to_string(),
        },
        200,
    )
}

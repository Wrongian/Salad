use std::{env, sync::Arc};

use aws_sdk_s3::primitives::ByteStream;
use serde::{Deserialize, Serialize};
use tide::{
    log::{error, info},
    Request,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    connectors::{
        buckets::file::{delete_s3_link_image, update_s3_link_image},
        db::{
            image::{create_link_image, delete_link_image, get_link_image},
            link::{get_user_link_by_id, link_id_belongs_to_user, reorder_link, update_link_by_id},
        },
    },
    helpers::{auth::get_session_user_id, params::extract_link_id_from_params},
    models::{images::InsertLinkImage, links::UpdateLink},
    types::{
        error::{AssociationErrors, Error, RequestErrors, S3Errors},
        response::Response,
        state::TideState,
    },
};

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

#[derive(Debug, Deserialize)]
struct ReorderLinksPayload {
    link_id: i32,
    new_position_id: Option<i32>,
}
// TODO: combine update link title, bio & href into the same endpoint
pub async fn update_link_title(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
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
        _ => return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response(),
    };

    // validate title
    match update_title.validate() {
        Err(e) => return Error::ValidationError(e).into_response(),
        _ => (),
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    // assert user link with link_id exists
    match link_id_belongs_to_user(&mut conn, link_id, user_id).await {
        Ok(result) => {
            if !result {
                return Error::DBAssociationError(AssociationErrors::LinkDoesNotBelongToUser)
                    .into_response();
            }
        }
        Err(e) => return Error::DieselError(e).into_response(),
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
        Err(e) => return Error::DieselError(e).into_response(),
    };
    Response::empty().into_response()
}

pub async fn update_link_bio(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // extract link id
    let link_id = match extract_link_id_from_params(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // extract title payload body
    let update_bio: UpdateBioPayload = match req.body_json().await {
        Ok(title_obj) => title_obj,
        _ => return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response(),
    };

    // validate title
    match update_bio.validate() {
        Err(e) => return Error::ValidationError(e).into_response(),
        _ => (),
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();
    // check user link with link_id exists
    match get_user_link_by_id(&mut conn, link_id, user_id).await {
        Ok(_) => (),
        _ => return Error::NotFoundError(String::from("Link")).into_response(),
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
        Err(err) => return Error::DieselError(err).into_response(),
    };
    Response::empty().into_response()
}

pub async fn update_link_href(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // extract link id
    let link_id = match extract_link_id_from_params(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // extract href payload body
    let updated_href: UpdateHrefPayload = match req.body_json().await {
        Ok(title_obj) => title_obj,
        _ => return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response(),
    };

    // validate href
    match updated_href.validate() {
        Err(e) => return Error::ValidationError(e).into_response(),
        _ => (),
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    // check user link with link_id exists
    match get_user_link_by_id(&mut conn, link_id, user_id).await {
        Ok(_) => (),
        _ => return Error::NotFoundError(String::from("Link")).into_response(),
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
        Err(e) => return Error::DieselError(e).into_response(),
    };

    Response::empty().into_response()
}

pub async fn update_link_picture(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // extract link id
    let link_id = match extract_link_id_from_params(&req) {
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

    // assert link_id belongs to user_id
    match link_id_belongs_to_user(&mut conn, link_id, user_id).await {
        Ok(is_user_link) => {
            if !is_user_link {
                return Error::DBAssociationError(AssociationErrors::LinkDoesNotBelongToUser)
                    .into_response();
            }
        }
        Err(e) => return Error::DieselError(e).into_response(),
    }

    // remove previous file; if any
    match get_link_image(&mut conn, link_id).await {
        Ok(img) => {
            // remove from s3 if present
            let result = delete_s3_link_image(s3_client, img.filename).await;
            let result_db = delete_link_image(&mut conn, link_id).await;
            if result.is_err() {
                error!("Error deleting link image: {}", result.unwrap_err());
            }

            if result_db.is_err() {
                error!("Error deleting link image: {:?}", result_db.err());
            }
        }
        // do nothing if not found
        Err(msg) => (),
    }

    // get uploaded file as bytes
    if bytes.is_err() {
        error!("failed to get bytes from request body.");
        return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response();
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
            return Error::S3Error(S3Errors::FailedToUploadImage).into_response();
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
        Ok(_) => Response::new(UploadLinkResponseBody { href: cdn_href }).into_response(),
        Err(e) => Error::DieselError(e).into_response(),
    }
}

pub async fn reorder_links(mut req: Request<Arc<TideState>>) -> tide::Result {
    // get user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // get reordering links
    let reorder_link_params: ReorderLinksPayload = match req.body_json().await {
        Ok(body) => body,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
        }
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    // assert both links belong to user_id
    match link_id_belongs_to_user(&mut conn, reorder_link_params.link_id, user_id).await {
        Ok(is_user_link) => {
            if !is_user_link {
                return Error::DBAssociationError(AssociationErrors::LinkDoesNotBelongToUser)
                    .into_response();
            }
        }
        Err(e) => return Error::DieselError(e).into_response(),
    }

    if reorder_link_params.new_position_id.is_some() {
        match link_id_belongs_to_user(
            &mut conn,
            reorder_link_params.new_position_id.unwrap(),
            user_id,
        )
        .await
        {
            Ok(is_user_link) => {
                if !is_user_link {
                    return Error::DBAssociationError(AssociationErrors::LinkDoesNotBelongToUser)
                        .into_response();
                }
            }
            Err(e) => return Error::DieselError(e).into_response(),
        };
    }

    // reorder links
    match reorder_link(
        &mut conn,
        reorder_link_params.link_id,
        reorder_link_params.new_position_id,
    )
    .await
    {
        Ok(_) => Response::empty().into_response(),
        Err(e) => Error::DieselError(e).into_response(),
    }
}

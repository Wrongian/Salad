use std::sync::Arc;

use tide::{log::error, Request};

use crate::{
    connectors::{
        buckets::file::delete_s3_link_image,
        db::{
            image::{delete_link_image, get_link_image},
            link::{delete_link_by_id, link_id_belongs_to_user},
        },
    },
    helpers::{auth::get_session_user_id, params::extract_link_id_from_params},
    types::{
        error::{AssociationErrors, Error, S3Errors},
        response::Response,
        state::TideState,
    },
};

pub async fn delete_link_picture(mut req: Request<Arc<TideState>>) -> tide::Result {
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

    // get image by id
    let img_obj = match get_link_image(&mut conn, link_id).await {
        Ok(img) => img,
        Err(e) => return Error::DieselError(e).into_response(),
    };

    // delete image from db
    match delete_link_by_id(&mut conn, link_id).await {
        Ok(_) => (),
        Err(e) => return Error::DieselError(e).into_response(),
    };

    // delete image from s3
    match delete_s3_link_image(s3_client, img_obj.filename).await {
        Ok(_) => Response::empty().into_response(),
        Err(e) => {
            error!("Unable to delete link from s3: {}", e);
            return Error::S3Error(S3Errors::FailedToDeleteImage).into_response();
        }
    }
}

pub async fn delete_links(req: Request<Arc<TideState>>) -> tide::Result {
    // get user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
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
    match link_id_belongs_to_user(&mut conn, link_id, user_id).await {
        Ok(is_user_link) => {
            if !is_user_link {
                return Error::DBAssociationError(AssociationErrors::LinkDoesNotBelongToUser)
                    .into_response();
            }
        }
        Err(e) => return Error::DieselError(e).into_response(),
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
        Ok(res) => Response::empty().into_response(),
        Err(e) => {
            error!("Error in deleting link: {:?}", e);
            Error::DieselError(e).into_response()
        }
    }
}

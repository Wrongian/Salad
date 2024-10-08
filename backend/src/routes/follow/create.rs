use std::sync::Arc;

use chrono::Utc;
use serde::Deserialize;
use tide::Request;

use crate::{
    connectors::db::{
        follow::{add_follow_request, has_follow_request, is_following},
        insight::update_user_insights,
        user::has_user_id,
    },
    helpers::{
        auth::get_session_user_id, notifications::create_request_notification,
        state::get_connection,
    },
    models::follows::InsertFollowRequest,
    models::insights::{Increment, UpdateUserInsight},
    types::{
        error::{AssociationErrors, Error, RequestErrors},
        response::Response,
        state::TideState,
    },
};

#[derive(Deserialize)]
pub struct CreateOutBoundFollowRequestPayload {
    pending_follow_id: i32,
}

pub async fn create_outbound_follow_request(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    let mut conn = get_connection(&mut req);

    // check user id exists in db
    match has_user_id(&mut conn, user_id).await {
        Ok(true) => (),
        Ok(false) => return Error::NotFoundError(String::from("User id")).into_response(),
        Err(e) => return Error::DieselError(e).into_response(),
    }

    let to_id = match req.body_json::<CreateOutBoundFollowRequestPayload>().await {
        Ok(payload) => payload.pending_follow_id,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
        }
    };

    // check to_id == user_id
    if to_id == user_id {
        return Error::AssociationError(AssociationErrors::InvalidFollowUser).into_response();
    }

    // check if user with to_id exists
    match has_user_id(&mut conn, to_id).await {
        Ok(true) => (),
        Ok(false) => return Error::NotFoundError(String::from("Following user")).into_response(),
        Err(e) => return Error::DieselError(e).into_response(),
    }

    // check if request record already exists, or already following
    let has_follow_request = match has_follow_request(&mut conn, to_id, user_id).await {
        Ok(res) => res,
        Err(e) => return Error::DieselError(e).into_response(),
    };
    let is_following = match is_following(&mut conn, user_id, to_id).await {
        Ok(res) => res,
        Err(e) => return Error::DieselError(e).into_response(),
    };
    if has_follow_request || is_following {
        return Error::AssociationError(AssociationErrors::InvalidFollowUser).into_response();
    }

    // add new record to db
    if let Err(e) = add_follow_request(
        &mut conn,
        &InsertFollowRequest {
            from_id: user_id,
            to_id,
        },
    )
    .await
    {
        return Error::DieselError(e).into_response();
    }

    // increment follow request analytics
    let increment_follow_requests =
        UpdateUserInsight::increment_follow_request_count(to_id, Utc::now().naive_utc());

    if let Err(e) = update_user_insights(&mut conn, increment_follow_requests).await {
        log::error!(
            "Failed to increment follow request count for user insights {:?}",
            e
        );
    }

    // TODO: publish notification
    // create notification
    match create_request_notification(&mut conn, to_id, user_id).await {
        Ok(_) => {}
        Err(e) => return e.into_response(),
    }

    return Response::empty().into_response();
}

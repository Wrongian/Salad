use std::sync::Arc;

use serde::Deserialize;
use tide::Request;

use crate::{
    connectors::db::{
        connection::DBConnection,
        follow::{
            delete_follow_request, delete_follower as db_delete_follower,
            delete_following as db_delete_following, has_follow_request, has_follower,
            is_following,
        },
        user::has_user_id,
    },
    helpers::{auth::get_session_user_id, state::get_connection},
    types::{
        error::{AssociationErrors, Error, RequestErrors},
        response::Response,
        state::TideState,
    },
};

#[derive(Deserialize)]
pub struct DeleteInboundFollowRequestPayload {
    pending_follower_id: i32,
}

#[derive(Deserialize)]
pub struct DeleteOutboundFollowRequestPayload {
    pending_follow_id: i32,
}

#[derive(Deserialize)]
pub struct DeleteFollowerPayload {
    follower_id: i32,
}
#[derive(Deserialize)]
pub struct DeleteFollowingPayload {
    following_id: i32,
}

pub async fn delete_inbound_follow_request(mut req: Request<Arc<TideState>>) -> tide::Result {
    let pending_follower_id = match req.body_json::<DeleteInboundFollowRequestPayload>().await {
        Ok(payload) => payload.pending_follower_id,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
        }
    };

    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    let mut conn = get_connection(&mut req);

    // check if request with the pending_follower_id exists
    match has_follow_request(&mut conn, user_id, pending_follower_id).await {
        Ok(true) => (),
        Ok(false) => return Error::NotFoundError(String::from("Follow Request")).into_response(),
        Err(e) => return Error::DieselError(e).into_response(),
    };

    // delete record from db (pending_follow_requests table)
    if let Err(e) = delete_follow_request(&mut conn, user_id, pending_follower_id).await {
        return Error::DieselError(e).into_response();
    }

    // TODO: delete tagged notifications

    return Response::empty().into_response();
}

pub async fn delete_outbound_follow_request(mut req: Request<Arc<TideState>>) -> tide::Result {
    let pending_follow_id = match req.body_json::<DeleteOutboundFollowRequestPayload>().await {
        Ok(payload) => payload.pending_follow_id,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
        }
    };
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    let mut conn: DBConnection = get_connection(&mut req);

    // check if request with the pending_follow_id exists
    match has_follow_request(&mut conn, pending_follow_id, user_id).await {
        Ok(true) => (),
        Ok(false) => return Error::NotFoundError(String::from("Follow Request")).into_response(),
        Err(e) => return Error::DieselError(e).into_response(),
    };

    // delete record from db (pending_follow_requests table)
    if let Err(e) = delete_follow_request(&mut conn, pending_follow_id, user_id).await {
        return Error::DieselError(e).into_response();
    }

    // TODO: delete tagged notifications

    return Response::empty().into_response();
}

pub async fn delete_follower(mut req: Request<Arc<TideState>>) -> tide::Result {
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

    let follower_id = match req.body_json::<DeleteFollowerPayload>().await {
        Ok(payload) => payload.follower_id,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
        }
    };

    // check if user with follower_id is a follower of user_id
    match has_follower(&mut conn, user_id, follower_id).await {
        Ok(true) => (),
        Ok(false) => return Error::NotFoundError(String::from("Follower")).into_response(),
        Err(e) => return Error::DieselError(e).into_response(),
    };

    // delete record
    if let Err(e) = db_delete_follower(&mut conn, user_id, follower_id).await {
        return Error::DieselError(e).into_response();
    }

    return Response::empty().into_response();
}

pub async fn delete_following(mut req: Request<Arc<TideState>>) -> tide::Result {
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

    let following_id = match req.body_json::<DeleteFollowingPayload>().await {
        Ok(payload) => payload.following_id,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
        }
    };

    // check if user with user_id follows following_id
    match is_following(&mut conn, user_id, following_id).await {
        Ok(true) => (),
        Ok(false) => {
            return Error::AssociationError(AssociationErrors::InvalidFollowUser).into_response()
        }
        Err(e) => return Error::DieselError(e).into_response(),
    }

    // delete record
    if let Err(e) = db_delete_following(&mut conn, user_id, following_id).await {
        return Error::DieselError(e).into_response();
    };

    return Response::empty().into_response();
}

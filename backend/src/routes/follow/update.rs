use std::sync::Arc;

use serde::Deserialize;
use tide::Request;

use crate::{
    connectors::db::{
        follow::{add_follow, delete_follow_request, has_follow_request},
        user::has_user_id,
    },
    helpers::{auth::get_session_user_id, state::get_connection},
    models::follows::InsertFollow,
    types::{
        error::{Error, RequestErrors},
        response::Response,
        state::TideState,
    },
};

#[derive(Deserialize)]
struct SettleInboundFollowRequestPayload {
    accept: bool,
    from_id: i32,
}

pub async fn settle_inbound_follow_request(mut req: Request<Arc<TideState>>) -> tide::Result {
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

    let SettleInboundFollowRequestPayload { accept, from_id } =
        match req.body_json::<SettleInboundFollowRequestPayload>().await {
            Ok(payload) => payload,
            Err(_) => {
                return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response()
            }
        };

    // check if pending follow request record exists
    match has_follow_request(&mut conn, user_id, from_id).await {
        Ok(true) => (),
        Ok(false) => return Error::NotFoundError(String::from("Follow request")).into_response(),
        Err(e) => return Error::DieselError(e).into_response(),
    };

    // delete the record from pending_follow_requests
    match delete_follow_request(&mut conn, user_id, from_id).await {
        Ok(_) => (),
        Err(e) => return Error::DieselError(e).into_response(),
    };

    // if accept, then add to followers; otherwise do nothing
    if accept {
        let follow = InsertFollow {
            from_id,
            to_id: user_id,
        };
        if let Err(e) = add_follow(&mut conn, &follow).await {
            return Error::DieselError(e).into_response();
        }
    }

    // TODO: publish notification

    return Response::empty().into_response();
}

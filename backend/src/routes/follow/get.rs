use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tide::Request;

use crate::{
    connectors::db::follow::{has_follow_request, is_following},
    helpers::{auth::get_session_user_id, state::get_connection},
    types::{
        error::{Error, RequestErrors},
        response::Response,
        state::TideState,
    },
};

#[derive(Deserialize)]
struct FollowStatusParams {
    pub id: i32,
}
#[derive(Serialize)]
struct FollowStatusResponsePayload {
    status: String,
}

impl FollowStatusResponsePayload {
    pub fn following() -> FollowStatusResponsePayload {
        FollowStatusResponsePayload {
            status: String::from("following"),
        }
    }

    pub fn none() -> FollowStatusResponsePayload {
        FollowStatusResponsePayload {
            status: String::from("none"),
        }
    }

    pub fn pending() -> FollowStatusResponsePayload {
        FollowStatusResponsePayload {
            status: String::from("pending"),
        }
    }

    pub fn into_response(self) -> tide::Result {
        Response::new(self).into_response()
    }
}

pub async fn get_follow_status(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    let to_id = match req.query::<FollowStatusParams>() {
        Ok(params) => params.id,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedParams).into_response()
        }
    };
    let mut conn = get_connection(&mut req);
    // check user is following to_id
    match is_following(&mut conn, user_id, to_id).await {
        Ok(true) => return FollowStatusResponsePayload::following().into_response(),
        Ok(false) => (),
        Err(e) => return Error::DieselError(e).into_response(),
    }

    // check pending request
    match has_follow_request(&mut conn, to_id, user_id).await {
        Ok(true) => FollowStatusResponsePayload::pending().into_response(),
        Ok(false) => FollowStatusResponsePayload::none().into_response(),
        Err(e) => Error::DieselError(e).into_response(),
    }
}

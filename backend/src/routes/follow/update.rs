use std::sync::Arc;

use chrono::Utc;
use serde::Deserialize;
use tide::Request;

use crate::connectors::db::notifications::{
    delete_notification_by_uids, notification_exists_by_uids,
};
use crate::helpers::notifications::{create_accepted_notification, FOLLOW_REQUEST_TYPE};
use crate::types::error::{Error, RequestErrors};
use crate::types::response::Response;
use crate::types::state::TideState;
use crate::{
    connectors::db::{
        follow::{add_follow, delete_follow_request, has_follow_request},
        insight::update_user_insights,
        user::has_user_id,
    },
    helpers::{auth::get_session_user_id, state::get_connection},
    models::{
        follows::InsertFollow,
        insights::{Increment, UpdateUserInsight},
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
        // update insight analytics
        let increment_follows =
            UpdateUserInsight::increment_follow_count(user_id, Utc::now().naive_utc());

        // fail silently
        if let Err(e) = update_user_insights(&mut conn, increment_follows).await {
            log::error!("Failed to increment follow count for user insights {:?}", e);
        }
    }

    // publish notification
    match create_accepted_notification(&mut conn, from_id, user_id).await {
        Ok(_) => {}
        Err(e) => return e.into_response(),
    }

    // check if notification exists
    match notification_exists_by_uids(&mut conn, from_id, user_id, FOLLOW_REQUEST_TYPE).await {
        Ok(true) => {
            // delete notification for user
            match delete_notification_by_uids(&mut conn, from_id, user_id, FOLLOW_REQUEST_TYPE)
                .await
            {
                Ok(_) => {}
                Err(e) => return e.into_response(),
            }
        }
        Ok(false) => {}

        Err(e) => return e.into_response(),
    }

    return Response::empty().into_response();
}

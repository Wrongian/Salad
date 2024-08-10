use std::sync::Arc;

use serde::Serialize;
use tide::Request;

use crate::{
    connectors::db::notifications::get_notifications_by_uid,
    helpers::auth::get_session_user_id,
    models::notifications::GetNotification,
    types::{response::Response, state::TideState},
};

#[derive(Debug, Serialize)]
struct GetNotificationsBody {
    notifications: Vec<GetNotification>,
}

pub async fn get_notifications(req: Request<Arc<TideState>>) -> tide::Result {
    // check if user is logged in
    let user_id = match get_session_user_id(&req) {
        Ok(uid) => uid,
        Err(_e) => {
            return Response::new(GetNotificationsBody {
                notifications: Vec::new(),
            })
            .into_response()
        }
    };

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    match get_notifications_by_uid(&mut conn, user_id).await {
        Ok(notifs) => Response::new(GetNotificationsBody {
            notifications: notifs,
        })
        .into_response(),
        Err(e) => return e.into_response(),
    }
}

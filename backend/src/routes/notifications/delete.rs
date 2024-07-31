use std::sync::Arc;

use tide::Request;

use crate::{
    connectors::db::{connection::DBConnection, notifications::clear_notifications},
    helpers::auth::get_session_user_id,
    types::{response::Response, state::TideState},
};

pub async fn delete_all_notifications(req: Request<Arc<TideState>>) -> tide::Result {
    // check if user is logged in
    let user_id = match get_session_user_id(&req) {
        Ok(uid) => uid,
        Err(e) => return e.into_response(),
    };

    let state = req.state();
    let mut conn: DBConnection = state.tide_pool.get().unwrap();

    // clear all notifications
    match clear_notifications(&mut conn, user_id).await {
        Ok(_) => return Response::empty().into_response(),
        Err(e) => return e.into_response(),
    }
}

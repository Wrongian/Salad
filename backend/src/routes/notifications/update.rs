use std::sync::Arc;

use serde::Deserialize;
use tide::Request;
use validator::Validate;

use crate::{
    connectors::db::{
        connection::DBConnection,
        notifications::{notification_belongs_to_user, notification_exists, update_notification},
    },
    helpers::auth::get_session_user_id,
    models::notifications::UpdateNotification,
    types::{
        error::{AssociationErrors, Error, RequestErrors},
        response::Response,
        state::TideState,
    },
};

#[derive(Debug, Deserialize, Validate)]
struct ReadNotificationPayload {
    notification_id: i32,
}

// read the notifications of the user
pub async fn read_notification(mut req: Request<Arc<TideState>>) -> tide::Result {
    // check if user is logged in
    let user_id = match get_session_user_id(&req) {
        Ok(uid) => uid,
        Err(e) => return e.into_response(),
    };

    // parse body and get notification ID
    // extract title payload body
    let read_notif: ReadNotificationPayload = match req.body_json().await {
        Ok(read_notification) => read_notification,
        _ => return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response(),
    };
    // validate
    match read_notif.validate() {
        Ok(_) => {}
        Err(e) => return Error::ValidationError(e).into_response(),
    }

    let state = req.state();
    let mut conn: DBConnection = state.tide_pool.get().unwrap();

    // check if notification exists
    match notification_exists(&mut conn, read_notif.notification_id).await {
        Ok(true) => {}
        Ok(false) => return Error::NotFoundError("Notification".to_string()).into_response(),
        Err(e) => return e.into_response(),
    }

    // check if notification belongs to the user
    match notification_belongs_to_user(&mut conn, read_notif.notification_id, user_id).await {
        Ok(true) => {}
        Ok(false) => {
            return Error::AssociationError(AssociationErrors::NotificationDoesNotBelongToUser)
                .into_response();
        }
        Err(e) => return e.into_response(),
    }

    let update_notif: UpdateNotification = UpdateNotification {
        user_id: None,
        trigger_id: None,
        notification_type: None,
        msg: None,
        is_read: Some(true),
    };
    // check if update the notification
    match update_notification(&mut conn, update_notif, read_notif.notification_id).await {
        Ok(_) => Response::empty().into_response(),
        Err(e) => return e.into_response(),
    }
}

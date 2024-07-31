use diesel::PgConnection;

use crate::connectors::db::notifications::create_notification;
use crate::models::users::GetUser;
use crate::types::error::Error;
use crate::{connectors::db::user::get_user_by_id, models::notifications::InsertNotification};

// notification creation helpers

// types
/*
Accepted: 1
Follow: 2
*/
pub const ACCEPTED_NOTIFICATION_TYPE: i32 = 1;
pub const FOLLOW_REQUEST_TYPE: i32 = 2;

fn accepted_notification_msg(trigger_name: String) -> String {
    let msg: String = trigger_name + " accepted your follow request";
    return msg;
}
fn accepted_notification(
    user_id: i32,
    trigger_id: i32,
    trigger_name: String,
) -> InsertNotification {
    let new_notif: InsertNotification = InsertNotification {
        user_id: user_id,
        trigger_id: trigger_id,
        is_read: false,
        created_at: chrono::Local::now().naive_local(),
        notification_type: ACCEPTED_NOTIFICATION_TYPE,
        msg: accepted_notification_msg(trigger_name),
    };
    return new_notif;
}
pub async fn create_accepted_notification(
    conn: &mut PgConnection,
    user_id: i32,
    trigger_id: i32,
) -> Result<(), Error> {
    let trigger_user: GetUser = match get_user_by_id(conn, trigger_id).await {
        Ok(u) => u,
        Err(e) => return Err(Error::DieselError(e)),
    };
    let new_notif: InsertNotification =
        accepted_notification(user_id, trigger_id, trigger_user.username);
    match create_notification(conn, new_notif).await {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}

fn request_notification_msg(trigger_name: String) -> String {
    let msg: String = trigger_name + " wants to follow you";
    return msg;
}
fn request_notification(user_id: i32, trigger_id: i32, trigger_name: String) -> InsertNotification {
    let new_notif: InsertNotification = InsertNotification {
        user_id: user_id,
        trigger_id: trigger_id,
        is_read: false,
        created_at: chrono::Local::now().naive_local(),
        notification_type: FOLLOW_REQUEST_TYPE,
        msg: request_notification_msg(trigger_name),
    };
    return new_notif;
}

pub async fn create_request_notification(
    conn: &mut PgConnection,
    user_id: i32,
    trigger_id: i32,
) -> Result<(), Error> {
    let trigger_user: GetUser = match get_user_by_id(conn, trigger_id).await {
        Ok(u) => u,
        Err(e) => return Err(Error::DieselError(e)),
    };
    let new_notif: InsertNotification =
        request_notification(user_id, trigger_id, trigger_user.username);
    match create_notification(conn, new_notif).await {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}

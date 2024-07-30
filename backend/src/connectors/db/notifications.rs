use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};

use crate::models::notifications::{GetNotification, InsertNotification, UpdateNotification};
use crate::types::error::Error;

/*
1. creates notification
2. updates notification via reading it
3. get the notifications by user (Return the first 10 unread, possibly sorted by date)
4. delete notification
5. clear all notifications
6. check if notification belongs to the user

 */

const NOTIFICATION_LIMIT: i64 = 20;

// create a link from a link model instance
pub async fn create_notification(
    conn: &mut PgConnection,
    notification: InsertNotification,
) -> Result<GetNotification, Error> {
    use crate::schema::notifications;
    match diesel::insert_into(notifications::table)
        .values(notification)
        .returning(GetNotification::as_returning())
        .get_result(conn)
    {
        Ok(notif) => return Ok(notif),
        Err(e) => Err(Error::DieselError(e)),
    }
}

// get all user links
pub async fn get_notifications_by_uid(
    conn: &mut PgConnection,
    uid: i32,
) -> Result<Vec<GetNotification>, Error> {
    use crate::schema::notifications::dsl::*;

    match notifications
        .filter(user_id.eq(uid))
        .limit(NOTIFICATION_LIMIT)
        .order(is_read.desc())
        .order(created_at.desc())
        .select(GetNotification::as_select())
        .load::<GetNotification>(conn)
    {
        Ok(notifs) => return Ok(notifs),
        Err(e) => return Err(Error::DieselError(e)),
    }
}

pub async fn notification_belongs_to_user(
    conn: &mut PgConnection,
    notif_id: i32,
    uid: i32,
) -> Result<bool, Error> {
    use crate::schema::notifications::dsl::*;
    notifications
        .filter(id.eq(notif_id).and(user_id.eq(uid)))
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
        .map_err(|e| Error::DieselError(e))
}

pub async fn update_notification(
    conn: &mut PgConnection,
    update_notif: UpdateNotification,
    notif_id: i32,
) -> Result<(), Error> {
    use crate::schema::notifications::dsl::*;
    diesel::update(notifications.filter(id.eq(notif_id)))
        .set(update_notif)
        .returning(id)
        .get_result::<i32>(conn)
        .map_err(|e| Error::DieselError(e))
        .map(|_| ())
}

pub async fn delete_notification(conn: &mut PgConnection, notif_id: i32) -> Result<(), Error> {
    use crate::schema::notifications::dsl::*;
    diesel::delete(notifications.filter(id.eq(notif_id)))
        .returning(id)
        .get_result::<i32>(conn)
        .map(|_| ())
        .map_err(|e| Error::DieselError(e))
}

pub async fn delete_notification_by_uids(
    conn: &mut PgConnection,
    tid: i32,
    uid: i32,
    notif_type: i32,
) -> Result<(), Error> {
    use crate::schema::notifications::dsl::*;
    diesel::delete(
        notifications.filter(
            user_id
                .eq(uid)
                .and(trigger_id.eq(tid))
                .and(notification_type.eq(notif_type)),
        ),
    )
    .returning(id)
    .get_result::<i32>(conn)
    .map(|_| ())
    .map_err(|e| Error::DieselError(e))
}

pub async fn notification_exists_by_uids(
    conn: &mut PgConnection,
    tid: i32,
    uid: i32,
    notif_type: i32,
) -> Result<bool, Error> {
    use crate::schema::notifications::dsl::*;
    notifications
        .filter(
            user_id
                .eq(uid)
                .and(trigger_id.eq(tid))
                .and(notification_type.eq(notif_type)),
        )
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
        .map_err(|e| Error::DieselError(e))
}

pub async fn clear_notifications(conn: &mut PgConnection, uid: i32) -> Result<(), Error> {
    use crate::schema::notifications::dsl::*;
    diesel::delete(notifications.filter(user_id.eq(uid)))
        .execute(conn)
        .map(|_| ())
        .map_err(|e| Error::DieselError(e))
}

pub async fn notification_exists(conn: &mut PgConnection, notif_id: i32) -> Result<bool, Error> {
    use crate::schema::notifications::dsl::*;
    notifications
        .filter(id.eq(notif_id))
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
        .map_err(|e| Error::DieselError(e))
}

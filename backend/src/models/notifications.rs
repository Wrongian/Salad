use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

// types of notifcation
/*
1. Friend Request Notification
2. Friend Acceptance Notification
3.


*/
#[derive(Queryable, Selectable, Insertable, Serialize, Debug, Clone)]
#[diesel(table_name = crate::schema::notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetNotification {
    pub id: i32,
    pub user_id: i32,
    pub trigger_id: i32,
    // pub created_at: NaiveDateTime,
    pub notification_type: i32,
    pub msg: String,
    pub is_read: bool,
}

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertNotification {
    pub user_id: i32,
    pub trigger_id: i32,
    pub created_at: NaiveDateTime,
    pub notification_type: i32,
    pub msg: String,
    pub is_read: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::notifications)]
pub struct UpdateNotification {
    pub user_id: Option<i32>,
    pub trigger_id: Option<i32>,
    pub notification_type: Option<i32>,
    pub msg: Option<String>,
    pub is_read: Option<bool>,
}

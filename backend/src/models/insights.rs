use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::user_insights)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetUserInsight {
    pub user_id: i32,
    pub view_count: i32,
    pub follow_count: i32,
    pub unfollow_count: i32,
    pub follow_request_count: i32,
    pub share_count: i32,
    pub created_bucket: NaiveDateTime,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = crate::schema::user_insights)]
pub struct UpdateUserInsight {
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub view_count: Option<i32>,
    pub follow_count: Option<i32>,
    pub unfollow_count: Option<i32>,
    pub follow_request_count: Option<i32>,
    pub share_count: Option<i32>,
}

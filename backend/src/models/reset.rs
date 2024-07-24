use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

// this is for email requests

// Create
#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::reset_password_request)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertRequest {
    pub code: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

// Get
#[derive(Queryable, Selectable, Insertable, Debug, Serialize)]
#[diesel(table_name = crate::schema::reset_password_request)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetRequest {
    pub created_at: NaiveDateTime,
    pub code: String,
    pub user_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::reset_password_request)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateRequest {
    pub code: String,
    pub user_id: Option<i32>,
}

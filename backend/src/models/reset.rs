use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

// for password resets
#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::reset_password_request)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertRequest {
    pub created_at: NaiveDateTime,
    pub code: String,
    pub user_id: i32,
}

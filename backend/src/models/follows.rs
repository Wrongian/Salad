use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};

#[derive(Insertable)]
#[diesel(table_name = crate::schema::follows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertFollow {
    pub from_id: i32,
    pub to_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::pending_follow_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertFollowRequest {
    pub from_id: i32,
    pub to_id: i32,
}
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::pending_follow_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetFollowRequest {
    pub from_id: i32,
    pub to_id: i32,
}

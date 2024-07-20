use diesel::{deserialize::Queryable, prelude::Insertable};

#[derive(Insertable)]
#[diesel(table_name = crate::schema::follows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertFollow {
    from_id: i32,
    to_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::pending_follow_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertFollowRequest {
    from_id: i32,
    to_id: i32,
}

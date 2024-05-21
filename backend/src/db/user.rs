use std::error::Error;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use crate::models::users::User;

pub async fn create(conn: &mut PgConnection, user: &User) {
    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("error");
}
use std::error::Error;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use crate::models::users::User;
use diesel::prelude::*;

pub async fn create(conn: &mut PgConnection, user: &User) {
    use crate::schema::users;
    diesel::insert_into(users::table)
        .values(user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("error");
}

pub async fn get_user_id_from_name(conn: &mut PgConnection, name: &String) -> i32 {
    use crate::schema::users::dsl::*;
    let res = users
        .filter(username.eq(name))
        .select(id)
        .first::<i32>(conn)
        .unwrap();

    return res;
}
use crate::models::users::{GetUser, InsertUser, UpdateUser, UserProfileView};
use crate::types::error::Error;
use diesel::prelude::*;
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper};

// create a user with a user instance
pub async fn create(conn: &mut PgConnection, user: &InsertUser) -> GetUser {
    use crate::schema::users;
    diesel::insert_into(users::table)
        .values(user)
        .returning(GetUser::as_returning())
        .get_result(conn)
        .expect("error")
}

// get a user id by their name
pub async fn get_user_id_from_name(conn: &mut PgConnection, name: &String) -> i32 {
    use crate::schema::users::dsl::*;
    let res = users
        .filter(username.eq(name))
        .select(id)
        .first::<i32>(conn)
        .unwrap();

    return res;
}

// get password and salt of user from their id
pub async fn get_password_salt_from_id(conn: &mut PgConnection, user_id: i32) -> (String, String) {
    use crate::schema::users::dsl::*;
    let res = users
        .find(user_id)
        .select((password, salt))
        .first::<(String, String)>(conn)
        .unwrap();

    return res;
}

// check uniqueness of username and email string before inserting
pub async fn check_user_exists(
    conn: &mut PgConnection,
    name: &String,
    email_string: &String,
) -> bool {
    use crate::schema::users::dsl::*;
    let count: i64 = users
        .filter(username.eq(&name))
        .or_filter(email.eq(&email_string))
        .count()
        .get_result::<i64>(conn)
        .unwrap();

    if count == 0 {
        return false;
    }
    true
}

// check if a username is already taken in the database
pub async fn check_username_present(conn: &mut PgConnection, name: &String) -> bool {
    use crate::schema::users::dsl::*;
    let count: i64 = users
        .filter(username.eq(&name))
        .count()
        .get_result::<i64>(conn)
        .unwrap();

    if count >= 1 {
        return true;
    }
    false
}

// check if user with id exists
pub async fn has_user_id(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::users::dsl::{id, users};
    users
        .filter(id.eq(user_id))
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
}

// get the user_profile using the user's username
pub async fn get_user_profile_by_username(
    conn: &mut PgConnection,
    name: &String,
) -> Result<UserProfileView, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    users
        .filter(username.eq(&name))
        .select(UserProfileView::as_select())
        .first::<UserProfileView>(conn)
}

pub async fn get_user_by_id(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<GetUser, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    users
        .filter(id.eq(user_id))
        .select(GetUser::as_select())
        .first::<GetUser>(conn)
}

pub async fn update_user_by_id(
    conn: &mut PgConnection,
    user_id: i32,
    update_user: &UpdateUser,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use diesel::query_dsl::methods::FilterDsl;

    diesel::update(FilterDsl::filter(users, id.eq(user_id)))
        .set(update_user)
        .returning(id)
        .get_result::<i32>(conn)
        .map(|v| v == user_id)
}

pub async fn get_user_from_email(
    conn: &mut PgConnection,
    email_str: String,
) -> Result<GetUser, Error> {
    use crate::schema::users::dsl::*;
    match users
        .filter(email.eq(&email_str))
        .select(GetUser::as_select())
        .first::<GetUser>(conn)
    {
        Ok(user) => Ok(user),
        Err(e) => Err(Error::DieselError(e)),
    }
}

pub async fn does_email_exist(conn: &mut PgConnection, email_str: String) -> Result<bool, Error> {
    use crate::schema::users::dsl::*;
    match users
        .filter(email.eq(&email_str))
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
    {
        Ok(count) => return Ok(count),
        Err(e) => return Err(Error::DieselError(e)),
    }
}

pub async fn does_username_exist(
    conn: &mut PgConnection,
    username_str: String,
) -> Result<bool, Error> {
    use crate::schema::users::dsl::*;
    match users
        .filter(username.eq(&username_str))
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
    {
        Ok(count) => return Ok(count),
        Err(e) => return Err(Error::DieselError(e)),
    }
}

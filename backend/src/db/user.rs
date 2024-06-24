use crate::models::users::{GetUser, InsertUser, UpdateUser, UserProfileView};
use diesel::prelude::*;
use diesel::result::Error;
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

// get the user_profile using the user's username
pub async fn get_user_profile_by_username(
    conn: &mut PgConnection,
    name: &String,
) -> Result<UserProfileView, String> {
    use crate::schema::users::dsl::*;

    let retrieved_obj: Result<(i32, String, Option<String>, String), Error> = users
        .filter(username.eq(&name))
        .select((id, username, bio, display_name))
        .first::<(i32, String, Option<String>, String)>(conn);

    match retrieved_obj {
        Ok(obj) => Ok(UserProfileView {
            id: obj.0,
            username: obj.1,
            bio: obj.2,
            display_name: obj.3,
            picture: String::from("this_picture_is_a_placeholder"), // empty placeholder for now
        }),
        Err(err) => match err {
            Error::NotFound => Err(String::from(
                "Unable to find user profile with the given username.",
            )),
            _ => Err(String::from(
                "Error occurred in querying database for user profile.",
            )),
        },
    }
}

pub async fn get_user_by_id(conn: &mut PgConnection, user_id: i32) -> Result<GetUser, String> {
    use crate::schema::users::dsl::*;
    let result: Result<GetUser, Error> = users
        .filter(id.eq(user_id))
        .select(GetUser::as_select())
        .first::<GetUser>(conn);
    result.map_err(|_| "could not find user with given id.".to_string())
}

pub async fn update_user_by_id(
    conn: &mut PgConnection,
    user_id: i32,
    update_user: &UpdateUser,
) -> Result<bool, String> {
    use crate::schema::users::dsl::*;
    use diesel::query_dsl::methods::FilterDsl;
    let update_user_id: Result<i32, Error> =
        diesel::update(FilterDsl::filter(users, id.eq(user_id)))
            .set(update_user)
            .returning(id)
            .get_result::<i32>(conn);

    update_user_id.map(|v| v == user_id).map_err(|e| match e {
        Error::NotFound => String::from("User does not exist."),
        _ => String::from("Failed to update profile."),
    })
}

use crate::models::users::{User, UserProfileView};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

// create a user with a user instance
pub async fn create(conn: &mut PgConnection, user: &User) {
    use crate::schema::users;
    diesel::insert_into(users::table)
        .values(user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("error");
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

    let retrieved_obj: Result<(String, Option<String>, String), Error> = users
        .filter(username.eq(&name))
        .select((username, bio, display_name))
        .first::<(String, Option<String>, String)>(conn);

    match retrieved_obj {
        Ok(obj) => Ok(UserProfileView {
            username: obj.0,
            bio: obj.1,
            display_name: obj.2,
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

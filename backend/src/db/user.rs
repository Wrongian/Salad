use crate::db::user;
use crate::models::users::{User, UserProfileView};
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

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

pub async fn get_password_salt_from_id(conn: &mut PgConnection, user_id: i32) -> (String, String) {
    use crate::schema::users::dsl::*;
    let res = users
        .find(user_id)
        .select((password, salt))
        .first::<(String, String)>(conn)
        .unwrap();

    return res;
}

// check uniqueness before inserting
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

pub async fn get_user_profile_by_username(
    conn: &mut PgConnection,
    name: &String,
) -> UserProfileView {
    use crate::schema::users::dsl::*;

    let retrieved_obj: (String, Option<String>, String) = users
        .filter(username.eq(&name))
        .select((username, bio, display_name))
        .first::<(String, Option<String>, String)>(conn)
        .unwrap();

    UserProfileView {
        username: retrieved_obj.0,
        bio: retrieved_obj.1,
        display_name: retrieved_obj.2,
        picture: String::from(""), // empty placeholder for now
    }
}

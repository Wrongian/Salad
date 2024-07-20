use diesel::{BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::models::follows::{InsertFollow, InsertFollowRequest};

pub async fn add_follow(
    conn: &mut PgConnection,
    follow: &InsertFollow,
) -> Result<(), diesel::result::Error> {
    use crate::schema::follows;

    diesel::insert_into(follows::table)
        .values(follow)
        .execute(conn)
        .map(|_| ())
}

pub async fn add_follow_request(
    conn: &mut PgConnection,
    follow_request: &InsertFollowRequest,
) -> Result<(), diesel::result::Error> {
    use crate::schema::pending_follow_requests;

    diesel::insert_into(pending_follow_requests::table)
        .values(follow_request)
        .execute(conn)
        .map(|_| ())
}

pub async fn delete_follower(
    conn: &mut PgConnection,
    user_id: i32,
    follower_id: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::follows::dsl::*;

    diesel::delete(follows.filter(to_id.eq(user_id).and(from_id.eq(follower_id))))
        .execute(conn)
        .map(|_| ())
}

pub async fn delete_following(
    conn: &mut PgConnection,
    user_id: i32,
    following_id: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::follows::dsl::*;

    diesel::delete(follows.filter(to_id.eq(following_id).and(from_id.eq(user_id))))
        .execute(conn)
        .map(|_| ())
}

pub async fn delete_follow_request(
    conn: &mut PgConnection,
    to_user_id: i32,
    from_user_id: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::pending_follow_requests::dsl::*;

    diesel::delete(
        pending_follow_requests.filter(to_id.eq(to_user_id).and(from_id.eq(from_user_id))),
    )
    .execute(conn)
    .map(|_| ())
}

pub async fn has_follower(
    conn: &mut PgConnection,
    user_id: i32,
    follower_id: i32,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::follows::dsl::*;
    follows
        .filter(to_id.eq(user_id).and(from_id.eq(follower_id)))
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
}

pub async fn is_following(
    conn: &mut PgConnection,
    user_id: i32,
    following_id: i32,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::follows::dsl::*;
    follows
        .filter(to_id.eq(following_id).and(from_id.eq(user_id)))
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
}

pub async fn has_follow_request(
    conn: &mut PgConnection,
    to_user_id: i32,
    from_user_id: i32,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::pending_follow_requests::dsl::*;
    pending_follow_requests
        .filter(to_id.eq(to_user_id).and(from_id.eq(from_user_id)))
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
}

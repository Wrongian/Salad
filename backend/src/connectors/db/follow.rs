use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, PgConnection,
    QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods,
};

use crate::{
    models::{
        follows::{GetFollowRequest, InsertFollow, InsertFollowRequest},
        images::GetImage,
        users::GetUser,
    },
    types::error::Error,
};

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

pub async fn get_follower_count(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::follows::dsl::*;

    follows
        .filter(to_id.eq(user_id))
        .count()
        .get_result::<i64>(conn)
}
pub async fn get_following_count(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::follows::dsl::*;

    follows
        .filter(from_id.eq(user_id))
        .count()
        .get_result::<i64>(conn)
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

pub async fn get_queried_followers(
    conn: &mut PgConnection,
    query: String,
    user_id: i32,
    index: i64,
    per_page: i64,
) -> Result<Vec<(GetUser, Option<GetImage>)>, diesel::result::Error> {
    use crate::schema::follows;
    use crate::schema::images;
    use crate::schema::users;

    follows::table
        .inner_join(
            users::table.on(follows::from_id
                .eq(users::id)
                .and(follows::to_id.eq(user_id))),
        )
        .left_join(images::table.on(follows::from_id.nullable().eq(images::user_id)))
        .filter(users::display_name.like(["%", query.as_str(), "%"].join("")))
        .offset((index - 1) * per_page)
        .limit(per_page)
        .select((GetUser::as_select(), Option::<GetImage>::as_select()))
        .load::<(GetUser, Option<GetImage>)>(conn)
}

pub async fn get_queried_follower_total_count(
    conn: &mut PgConnection,
    user_id: i32,
    query: String,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::follows;
    use crate::schema::users;

    follows::table
        .inner_join(
            users::table.on(follows::from_id
                .eq(users::id)
                .and(follows::to_id.eq(user_id))),
        )
        .filter(users::display_name.like(["%", query.as_str(), "%"].join("")))
        .count()
        .get_result::<i64>(conn)
}
pub async fn get_queried_following_total_count(
    conn: &mut PgConnection,
    user_id: i32,
    query: String,
) -> Result<i64, diesel::result::Error> {
    use crate::schema::follows;
    use crate::schema::users;

    follows::table
        .inner_join(
            users::table.on(follows::to_id
                .eq(users::id)
                .and(follows::from_id.eq(user_id))),
        )
        .filter(users::display_name.like(["%", query.as_str(), "%"].join("")))
        .count()
        .get_result::<i64>(conn)
}
pub async fn get_queried_pending_follow_request_total_count(
    conn: &mut PgConnection,
    user_id: i32,
    query: String,
) -> Result<i64, Error> {
    use crate::schema::pending_follow_requests;
    use crate::schema::users;
    pending_follow_requests::table
        .inner_join(
            users::table.on(pending_follow_requests::to_id
                .eq(users::id)
                .or(pending_follow_requests::from_id.eq(users::id))),
        )
        .filter(
            pending_follow_requests::from_id
                .eq(user_id)
                .or(pending_follow_requests::to_id.eq(user_id)),
        )
        .filter(users::id.ne(user_id))
        .filter(users::display_name.like(["%", query.as_str(), "%"].join("")))
        .count()
        .get_result::<i64>(conn)
        .map_err(|e| Error::DieselError(e))
}

pub async fn get_queried_pending_follow_requests(
    conn: &mut PgConnection,
    query: String,
    user_id: i32,
    index: i64,
    per_page: i64,
) -> Result<Vec<(GetUser, Option<GetImage>, GetFollowRequest)>, Error> {
    use crate::schema::images;
    use crate::schema::pending_follow_requests;
    use crate::schema::users;

    pending_follow_requests::table
        .inner_join(
            users::table.on(pending_follow_requests::to_id
                .eq(users::id)
                .or(pending_follow_requests::from_id.eq(users::id))),
        )
        .left_join(images::table.on(users::id.nullable().eq(images::user_id)))
        .filter(users::id.ne(user_id))
        .filter(
            pending_follow_requests::to_id
                .eq(user_id)
                .or(pending_follow_requests::from_id.eq(user_id)),
        )
        .filter(users::display_name.like(["%", query.as_str(), "%"].join("")))
        .offset((index - 1) * per_page)
        .limit(per_page)
        .select((
            GetUser::as_select(),
            Option::<GetImage>::as_select(),
            GetFollowRequest::as_select(),
        ))
        .load::<(GetUser, Option<GetImage>, GetFollowRequest)>(conn)
        .map_err(|e| Error::DieselError(e))
}

pub async fn get_queried_followings(
    conn: &mut PgConnection,
    query: String,
    user_id: i32,
    index: i64,
    per_page: i64,
) -> Result<Vec<(GetUser, Option<GetImage>)>, diesel::result::Error> {
    use crate::schema::follows;
    use crate::schema::images;
    use crate::schema::users;

    follows::table
        .inner_join(
            users::table.on(follows::to_id
                .eq(users::id)
                .and(follows::from_id.eq(user_id))),
        )
        .left_join(images::table.on(follows::to_id.nullable().eq(images::user_id)))
        .filter(users::display_name.like(["%", query.as_str(), "%"].join("")))
        .offset((index - 1) * per_page)
        .limit(per_page)
        .select((GetUser::as_select(), Option::<GetImage>::as_select()))
        .load::<(GetUser, Option<GetImage>)>(conn)
}

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    models::insights::{GetUserInsight, UpdateUserInsight},
    types::error::Error,
};

pub async fn update_user_insights(
    conn: &mut PgConnection,
    update_user_insight: UpdateUserInsight,
) -> Result<(), Error> {
    use crate::schema::user_insights::dsl::*;
    diesel::insert_into(user_insights)
        .values(&update_user_insight)
        .on_conflict((created_bucket, user_id))
        .do_update()
        .set((
            view_count.eq(view_count + update_user_insight.view_count.unwrap_or(0)),
            follow_count.eq(follow_count + update_user_insight.follow_count.unwrap_or(0)),
            unfollow_count.eq(unfollow_count + update_user_insight.unfollow_count.unwrap_or(0)),
            follow_request_count
                .eq(follow_request_count + update_user_insight.follow_request_count.unwrap_or(0)),
            share_count.eq(share_count + update_user_insight.share_count.unwrap_or(0)),
        ))
        .execute(conn)
        .map(|_| ())
        .map_err(|e| Error::DieselError(e))
}

pub async fn get_user_insights(
    conn: &mut PgConnection,
    user_id: i32,
) -> Result<Vec<GetUserInsight>, Error> {
    use crate::schema::user_insights;
    user_insights::table
        .filter(user_insights::user_id.eq(user_id))
        .select(GetUserInsight::as_select())
        .load::<GetUserInsight>(conn)
        .map_err(|e| Error::DieselError(e))
}

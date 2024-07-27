use diesel::sql_types::{Integer, Nullable};
use diesel::{
    BoolExpressionMethods, Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};

use crate::models::images::GetImage;
use crate::models::links::{GetLink, InsertLink, UpdateLink};
use crate::schema;

// create a link from a link model instance
pub async fn create(
    conn: &mut PgConnection,
    link: &InsertLink,
) -> Result<GetLink, diesel::result::Error> {
    use crate::schema::links;
    diesel::insert_into(links::table)
        .values(link)
        .returning(GetLink::as_returning())
        .get_result(conn)
}

pub async fn reorder_link(
    conn: &mut PgConnection,
    curr_link_id: i32,
    new_position_id: Option<i32>,
) -> Result<(), diesel::result::Error> {
    conn.transaction(|c| {
        diesel::sql_query("SELECT reorder_link($1, $2)")
            .bind::<Integer, _>(curr_link_id)
            .bind::<Nullable<Integer>, _>(new_position_id)
            .execute(c)
    })
    .map(|_| ())
}

// get link by id
pub async fn get_link_by_id(
    conn: &mut PgConnection,
    link_id: i32,
) -> Result<GetLink, diesel::result::Error> {
    use crate::schema::links::dsl::*;
    links
        .filter(id.eq(link_id))
        .select(GetLink::as_select())
        .first::<GetLink>(conn)
}

// get all user links
pub async fn get_user_links_by_id(
    conn: &mut PgConnection,
    userid: i32,
) -> Result<Vec<(GetLink, Option<GetImage>)>, diesel::result::Error> {
    use crate::schema::images::dsl;
    use crate::schema::links::dsl::*;

    diesel::QueryDsl::left_join(links.filter(user_id.eq(userid)), dsl::images)
        .select((GetLink::as_select(), Option::<GetImage>::as_select()))
        .distinct_on(schema::links::id)
        .load::<(GetLink, Option<GetImage>)>(conn)
}

// get user link by id
pub async fn get_user_link_by_id(
    conn: &mut PgConnection,
    link_id: i32,
    userid: i32,
) -> Result<GetLink, diesel::result::Error> {
    use crate::schema::links::dsl::*;
    links
        .filter(id.eq(link_id).and(user_id.eq(userid)))
        .select(GetLink::as_select())
        .first::<GetLink>(conn)
}

pub async fn link_id_belongs_to_user(
    conn: &mut PgConnection,
    link_id: i32,
    userid: i32,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::links::dsl::*;
    links
        .filter(id.eq(link_id).and(user_id.eq(userid)))
        .count()
        .get_result::<i64>(conn)
        .map(|count| count > 0)
}

pub async fn update_link_by_id(
    conn: &mut PgConnection,
    update_link: &UpdateLink,
    link_id: i32,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::links::dsl::*;
    diesel::update(links.filter(id.eq(link_id)))
        .set(update_link)
        .returning(id)
        .get_result::<i32>(conn)
        .map(|v| v == link_id)
}

pub async fn delete_link_by_id(
    conn: &mut PgConnection,
    link_id: i32,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::links::dsl::*;
    diesel::delete(links.filter(id.eq(link_id)))
        .returning(id)
        .get_result::<i32>(conn)
        .map(|res| res == link_id)
}

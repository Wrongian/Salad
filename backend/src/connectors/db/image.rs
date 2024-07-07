use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper};

use diesel::query_dsl::methods::{FilterDsl, SelectDsl};

use crate::models::images::{GetImage, InsertLinkImage, InsertProfileImage, UpdateImage};

pub async fn create_profile_image(
    conn: &mut PgConnection,
    image: &InsertProfileImage,
) -> Result<GetImage, diesel::result::Error> {
    use crate::schema::images;

    diesel::insert_into(images::table)
        .values(image)
        .returning(GetImage::as_returning())
        .get_result(conn)
}

pub async fn create_link_image(
    conn: &mut PgConnection,
    image: &InsertLinkImage,
) -> Result<GetImage, diesel::result::Error> {
    use crate::schema::images;

    diesel::insert_into(images::table)
        .values(image)
        .returning(GetImage::as_returning())
        .get_result(conn)
}

pub async fn get_profile_image(
    conn: &mut PgConnection,
    user_id_query: i32,
) -> Result<GetImage, diesel::result::Error> {
    use crate::schema::images::dsl::*;
    images
        .filter(user_id.eq(user_id_query))
        .select(GetImage::as_select())
        .first::<GetImage>(conn)
}

pub async fn get_link_image(
    conn: &mut PgConnection,
    link_id_query: i32,
) -> Result<GetImage, diesel::result::Error> {
    use crate::schema::images::dsl::*;
    images
        .filter(link_id.eq(link_id_query))
        .select(GetImage::as_select())
        .first::<GetImage>(conn)
}

pub async fn update_profile_image(
    conn: &mut PgConnection,
    update_image_query: &UpdateImage,
    user_id_query: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::images::dsl::*;
    diesel::update(images.filter(user_id.eq(user_id_query)))
        .set(update_image_query)
        .execute(conn)
        .map(|_| ())
}

pub async fn update_link_image(
    conn: &mut PgConnection,
    update_image_query: &UpdateImage,
    user_id_query: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::images::dsl::*;
    diesel::update(images.filter(user_id.eq(user_id_query)))
        .set(update_image_query)
        .execute(conn)
        .map(|_| ())
}

pub async fn delete_profile_image(
    conn: &mut PgConnection,
    user_id_query: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::images::dsl::*;
    diesel::delete(images.filter(user_id.eq(user_id_query)))
        .execute(conn)
        .map(|_| ())
}

pub async fn delete_link_image(
    conn: &mut PgConnection,
    link_id_query: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::images::dsl::*;
    // Ok(usize) -> usize is the number of rows affected
    diesel::delete(images.filter(link_id.eq(link_id_query)))
        .execute(conn)
        .map(|_| ())
}

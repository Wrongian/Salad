use diesel::result::DatabaseErrorKind;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper,
};

use diesel::query_dsl::methods::{FilterDsl, SelectDsl};

use crate::models::images::{GetImage, InsertLinkImage, InsertProfileImage, UpdateImage};

pub async fn create_profile_image(
    conn: &mut PgConnection,
    image: &InsertProfileImage,
) -> Result<GetImage, String> {
    use crate::schema::images;
    use diesel::result::Error;

    let result: Result<GetImage, Error> = diesel::insert_into(images::table)
        .values(image)
        .returning(GetImage::as_returning())
        .get_result(conn);

    result.map_err(|e| match e {
        Error::DatabaseError(kind, _) => match kind {
            DatabaseErrorKind::NotNullViolation => String::from("An internal error has occurred."),
            _ => String::from("An error has occurred in creating profile image."),
        },
        _ => String::from("An error occurred when creating profile image."),
    })
}

pub async fn create_link_image(
    conn: &mut PgConnection,
    image: &InsertLinkImage,
) -> Result<GetImage, String> {
    use crate::schema::images;
    use diesel::result::Error;
    let result: Result<GetImage, Error> = diesel::insert_into(images::table)
        .values(image)
        .returning(GetImage::as_returning())
        .get_result(conn);

    // TODO: abstraction for database error handling logic
    result.map_err(|e| match e {
        Error::DatabaseError(kind, _) => match kind {
            DatabaseErrorKind::NotNullViolation => String::from("An internal error has occurred."),
            _ => String::from("An error has occurred in creating link image."),
        },
        _ => String::from("An error occurred when creating link image."),
    })
}

pub async fn get_profile_image(
    conn: &mut PgConnection,
    user_id_query: i32,
) -> Result<GetImage, String> {
    use crate::schema::images::dsl::*;
    use diesel::result::Error;
    let result: Result<GetImage, Error> = images
        .filter(user_id.eq(user_id_query))
        .select(GetImage::as_select())
        .first::<GetImage>(conn);

    return result.map_err(|e| match e {
        _ => String::from("Failed to get profile image."),
    });
}

pub async fn get_link_image(
    conn: &mut PgConnection,
    link_id_query: i32,
) -> Result<GetImage, String> {
    use crate::schema::images::dsl::*;
    use diesel::result::Error;
    let result: Result<GetImage, Error> = images
        .filter(link_id.eq(link_id_query))
        .select(GetImage::as_select())
        .first::<GetImage>(conn);

    return result.map_err(|e| match e {
        _ => String::from("Failed to get profile image."),
    });
}

pub async fn update_profile_image(
    conn: &mut PgConnection,
    update_image_query: &UpdateImage,
    user_id_query: i32,
) -> Result<(), String> {
    use crate::schema::images::dsl::*;
    use diesel::result::Error;
    let result: Result<usize, Error> = diesel::update(images.filter(user_id.eq(user_id_query)))
        .set(update_image_query)
        .execute(conn);

    return result.map(|_| ()).map_err(|e| match e {
        _ => String::from("Failed to get profile image."),
    });
}

pub async fn update_link_image(
    conn: &mut PgConnection,
    update_image_query: &UpdateImage,
    user_id_query: i32,
) -> Result<(), String> {
    use crate::schema::images::dsl::*;
    use diesel::result::Error;
    let result: Result<usize, Error> = diesel::update(images.filter(user_id.eq(user_id_query)))
        .set(update_image_query)
        .execute(conn);

    return result.map(|_| ()).map_err(|e| match e {
        _ => String::from("Failed to get profile image."),
    });
}

pub async fn delete_profile_image(
    conn: &mut PgConnection,
    user_id_query: i32,
) -> Result<(), String> {
    use crate::schema::images::dsl::*;
    let result: Result<usize, diesel::result::Error> =
        diesel::delete(images.filter(user_id.eq(user_id_query))).execute(conn);
    return result
        .map(|_| ())
        .map_err(|_| "Error occurred in deleting the link.".to_string());
}

pub async fn delete_link_image(conn: &mut PgConnection, link_id_query: i32) -> Result<(), String> {
    use crate::schema::images::dsl::*;
    // Ok(usize) -> usize is the number of rows affected
    let result: Result<usize, diesel::result::Error> =
        diesel::delete(images.filter(link_id.eq(link_id_query))).execute(conn);
    return result
        .map(|_| ())
        .map_err(|_| "Error occurred in deleting the link.".to_string());
}

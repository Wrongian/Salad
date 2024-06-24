use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper,
};

use diesel::query_dsl::methods::{FilterDsl, SelectDsl};

use crate::models::links::{self, GetLink, InsertLink, UpdateLink, UpdateLinkImageSource};

// create a link from a link model instance
pub async fn create(conn: &mut PgConnection, link: &InsertLink) -> GetLink {
    use crate::schema::links;
    diesel::insert_into(links::table)
        .values(link)
        .returning(GetLink::as_returning())
        .get_result(conn)
        .expect("error in creating link")
}

// get link by id
pub async fn get_link_by_id(conn: &mut PgConnection, link_id: i32) -> Result<GetLink, String> {
    use crate::schema::links::dsl::*;
    let result: Result<GetLink, diesel::result::Error> = links
        .filter(id.eq(link_id))
        .select(GetLink::as_select())
        .first::<GetLink>(conn);
    result.map_err(|_| "Could not find the link.".to_string())
}

// get user link by id
pub async fn get_user_link_by_id(
    conn: &mut PgConnection,
    link_id: i32,
    users_id: i32,
) -> Result<GetLink, String> {
    use crate::schema::links::dsl::*;
    let result: Result<GetLink, diesel::result::Error> = links
        .filter(id.eq(link_id).and(user_id.eq(users_id)))
        .select(GetLink::as_select())
        .first::<GetLink>(conn);
    result.map_err(|_| "Could not find the link.".to_string())
}

pub async fn update_link_by_id(
    conn: &mut PgConnection,
    update_link: &UpdateLink,
    link_id: i32,
) -> Result<bool, String> {
    use crate::schema::links::dsl::*;
    let updated_link_id: Result<i32, diesel::result::Error> =
        diesel::update(links.filter(id.eq(link_id)))
            .set(update_link)
            .returning(id)
            .get_result::<i32>(conn);

    updated_link_id
        .map(|v| v == link_id)
        .map_err(|_| "Unable to update link by id.".to_string())
}

// TODO: abstract this method with above using a generic UpdateModel Type.
pub async fn update_link_img_by_id(
    conn: &mut PgConnection,
    update_link_img: &UpdateLinkImageSource,
    link_id: i32,
) -> Result<bool, String> {
    use crate::schema::links::dsl::*;
    let updated_link_id: Result<i32, diesel::result::Error> =
        diesel::update(links.filter(id.eq(link_id)))
            .set(update_link_img)
            .returning(id)
            .get_result::<i32>(conn);
    updated_link_id
        .map(|v| v == link_id)
        .map_err(|_| "Unable to update link by id.".to_string())
}

pub async fn delete_link_by_id(conn: &mut PgConnection, link_id: i32) -> Result<bool, String> {
    use crate::schema::links::dsl::*;
    let result: Result<i32, diesel::result::Error> = diesel::delete(links.filter(id.eq(link_id)))
        .returning(id)
        .get_result::<i32>(conn);
    return result
        .map(|res| res == link_id)
        .map_err(|_| "Error occurred in deleting the link.".to_string());
}

#[cfg(test)]
mod unit_test {
    use std::env;

    use diesel::{Connection, PgConnection};
    use dotenvy::dotenv;
    use random_string::generate;

    use crate::{
        db,
        models::{
            links::{GetLink, InsertLink},
            users::{GetUser, InsertUser},
        },
        schema::users,
    };
    use diesel::prelude::*;
    // NOTE: execute 'diesel migration run' before unit tests to ensure the tables are loaded into psql
    // before running the unit tests.

    pub async fn mock_connection() -> PgConnection {
        dotenv().expect("No .env file found");

        let database_url = env::var("DATABASE_URL").expect("No database url found");

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    pub async fn delete_mock_user(user_id: i32) {
        let mut conn = mock_connection().await;
        use crate::schema::users::dsl::*;
        let result: Result<i32, diesel::result::Error> =
            diesel::delete(users.filter(id.eq(user_id)))
                .returning(id)
                .get_result::<i32>(&mut conn);
        assert!(result.is_ok());
    }

    pub async fn create_mock_user() -> GetUser {
        let mut conn = mock_connection().await;
        let charset = "abcdefghijklmnopqrs1234567890";
        let user = InsertUser {
            username: generate(5, charset),
            display_name: "test-display-name".to_string(),
            password: "12345".to_string(),
            email: format!("{}@gmail.com", generate(5, charset)),
            bio: Some("this is a bio".to_string()),
            is_private: false,
            salt: "123".to_string(),
        };
        db::user::create(&mut conn, &user).await
    }

    pub async fn create_mock_link(user_id: i32) -> GetLink {
        let mut conn = mock_connection().await;

        let link = InsertLink {
            user_id,
            next_id: None,
            prev_id: None,
            description: None,
            title: None,
            href: "http://test-mock.com".to_string(),
        };
        db::link::create(&mut conn, &link).await
    }

    #[tokio::test]
    pub async fn it_creates_link_in_db() {
        let mut conn = mock_connection().await;

        let user = create_mock_user().await;
        let link = InsertLink {
            user_id: user.id,
            next_id: None,
            prev_id: None,
            description: None,
            title: None,
            href: "http://test-mock.com".to_string(),
        };
        let link = db::link::create(&mut conn, &link).await;
        assert!(true);

        // deletes properly
        assert!(db::link::delete_link_by_id(&mut conn, link.id)
            .await
            .unwrap());
        delete_mock_user(user.id).await;
    }
    #[tokio::test]
    pub async fn it_gets_link_by_id() {
        let mut conn = mock_connection().await;
        let user = create_mock_user().await;
        let mock_link = create_mock_link(user.id).await;
        let link = db::link::get_link_by_id(&mut conn, mock_link.id).await;
        assert!(link.is_ok());
        if link.is_err() {
            println!("Error in getting link: {:?}", link.err());
        }

        // deletes properly
        assert!(db::link::delete_link_by_id(&mut conn, mock_link.id)
            .await
            .unwrap());
        delete_mock_user(user.id).await;
    }
    #[tokio::test]
    pub async fn it_gets_user_link_by_id() {
        let mut conn = mock_connection().await;
        let user = create_mock_user().await;
        let mock_link = create_mock_link(user.id).await;
        let link = db::link::get_user_link_by_id(&mut conn, mock_link.id, user.id).await;
        assert!(link.is_ok());
        if link.is_err() {
            println!("Error in getting link: {:?}", link.err());
        }

        // deletes properly
        assert!(db::link::delete_link_by_id(&mut conn, mock_link.id)
            .await
            .unwrap());
        delete_mock_user(user.id).await;
    }
}

pub mod email;
pub mod link;
pub mod password_reset;
pub mod testing;

use random_string::generate;

use crate::models::{
    links::{GetLink, InsertLink},
    users::{GetUser, InsertUser},
};

use crate::connectors::db;
use diesel::prelude::*;
// NOTE: execute 'diesel migration run' before unit tests to ensure the tables are loaded into psql
// before running the unit tests.

use crate::connectors::db::mock_connection;

pub async fn delete_mock_user(user_id: i32) {
    let mut conn = mock_connection().await;
    use crate::schema::users::dsl::*;
    let result: Result<i32, diesel::result::Error> = diesel::delete(users.filter(id.eq(user_id)))
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
        description: None,
        title: None,
        href: "http://test-mock.com".to_string(),
    };
    db::link::create(&mut conn, &link).await.unwrap()
}

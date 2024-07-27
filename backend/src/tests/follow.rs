#[cfg(test)]
mod test {
    use std::env;

    use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
    use dotenvy::dotenv;
    use random_string::generate;

    use crate::{
        connectors::db::{
            self,
            follow::{delete_follower, get_follower_count, get_queried_followers},
        },
        models::{
            follows::InsertFollow,
            users::{GetUser, InsertUser},
        },
    };

    pub async fn mock_connection() -> PgConnection {
        dotenv().expect("No .env file found");

        let database_url = env::var("DATABASE_URL").expect("No database url found");

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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

    pub async fn delete_mock_user(user_id: i32) {
        let mut conn = mock_connection().await;
        use crate::schema::users::dsl::*;
        let result = diesel::delete(users.filter(id.eq(user_id)))
            .returning(id)
            .get_result::<i32>(&mut conn);
        assert!(result.is_ok());
    }

    pub async fn delete_follow_record(from_id: i32, to_id: i32) {
        let mut conn = mock_connection().await;
        let result = delete_follower(&mut conn, to_id, from_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    pub async fn it_gets_queried_followers() {
        use crate::connectors::db::follow::add_follow;
        // setup 3 users
        let user1 = create_mock_user().await;
        let user2 = create_mock_user().await;
        let user3 = create_mock_user().await;

        let mut conn = mock_connection().await;

        // user2 follows user1
        assert!(add_follow(
            &mut conn,
            &InsertFollow {
                from_id: user2.id,
                to_id: user1.id,
            },
        )
        .await
        .is_ok());

        // user3 follows user1
        assert!(add_follow(
            &mut conn,
            &InsertFollow {
                from_id: user3.id,
                to_id: user1.id,
            },
        )
        .await
        .is_ok());

        assert_eq!(
            get_follower_count(&mut conn, user1.id).await.unwrap(),
            2,
            "get_follower_count does not return 2!"
        );

        let results = get_queried_followers(&mut conn, String::from("test-"), user1.id, 1, 2).await;

        assert!(results.is_ok());
        let users = results.unwrap();
        assert_eq!(
            users.len(),
            2,
            "get_queried_followers does not return 2 followers!"
        );
        // check followers are distinct
        assert_ne!(users[0].0.id, users[1].0.id);

        // check followers are valid
        assert!([user2.id, user3.id].contains(&users[0].0.id));
        assert!([user2.id, user3.id].contains(&users[1].0.id));

        // clean up
        delete_follow_record(user2.id, user1.id).await;
        delete_follow_record(user3.id, user1.id).await;
        delete_mock_user(user1.id).await;
        delete_mock_user(user2.id).await;
        delete_mock_user(user3.id).await;
    }
}

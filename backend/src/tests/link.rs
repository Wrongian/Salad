#[cfg(test)]
mod link_tests {

    use crate::models::links::InsertLink;

    use crate::connectors::db;
    // NOTE: execute 'diesel migration run' before unit tests to ensure the tables are loaded into psql
    // before running the unit tests.

    use crate::connectors::db::mock_connection;
    use crate::tests::{create_mock_link, create_mock_user, delete_mock_user};

    #[tokio::test]
    pub async fn it_creates_link_in_db() {
        let mut conn = mock_connection().await;

        let user = create_mock_user().await;
        let link = InsertLink {
            user_id: user.id,
            next_id: None,
            description: None,
            title: None,
            href: "http://test-mock.com".to_string(),
        };
        let link = db::link::create(&mut conn, &link).await;
        assert!(link.is_ok());

        // deletes properly
        assert!(db::link::delete_link_by_id(&mut conn, link.unwrap().id)
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

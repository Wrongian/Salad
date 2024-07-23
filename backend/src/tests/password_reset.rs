#[cfg(test)]
mod password_reset_tests {

    use crate::connectors::db::mock_connection;
    use crate::connectors::db::reset::{
        create_request, delete_request, get_request_by_id, replace_request, request_exists,
    };
    use crate::models::reset::{InsertRequest, UpdateRequest};
    use crate::tests::{create_mock_user, delete_mock_user};

    #[tokio::test]
    pub async fn test_create_delete() {
        let mut conn = mock_connection().await;
        let user = create_mock_user().await;
        let request = InsertRequest {
            code: "anime".to_string(),
            user_id: user.id,
        };
        let res = create_request(&mut conn, request).await;
        assert!(res.is_ok());
        let res = delete_request(&mut conn, user.id).await;
        assert!(res.is_ok());
        delete_mock_user(user.id).await;
    }

    #[tokio::test]
    pub async fn test_request_exists() {
        let mut conn = mock_connection().await;
        let user = create_mock_user().await;
        let request = InsertRequest {
            code: "anime".to_string(),
            user_id: user.id,
        };
        let res = create_request(&mut conn, request).await;
        assert!(res.is_ok());
        assert!(request_exists(&mut conn, user.id).await.unwrap() == true);
        let res = delete_request(&mut conn, user.id).await;
        assert!(request_exists(&mut conn, user.id).await.unwrap() == false);
        assert!(res.is_ok());
        delete_mock_user(user.id).await;
    }

    #[tokio::test]
    pub async fn test_get_request() {
        let mut conn = mock_connection().await;
        let user = create_mock_user().await;
        let request = InsertRequest {
            code: "anime".to_string(),
            user_id: user.id,
        };
        let res = create_request(&mut conn, request).await;
        assert!(res.is_ok());
        let res = get_request_by_id(&mut conn, user.id).await;
        assert!(res.is_ok());
        assert!(res.unwrap().user_id == user.id);
        let res = delete_request(&mut conn, user.id).await;
        assert!(res.is_ok());
        delete_mock_user(user.id).await;
    }

    #[tokio::test]
    pub async fn test_replace_request() {
        let mut conn = mock_connection().await;
        let user = create_mock_user().await;
        let request = InsertRequest {
            code: "anime".to_string(),
            user_id: user.id,
        };
        let res = create_request(&mut conn, request).await;
        assert!(res.is_ok());
        let new_request = UpdateRequest {
            code: "bruh".to_string(),
            user_id: None,
        };
        let res = replace_request(&mut conn, user.id, new_request).await;
        assert!(res.is_ok());
        let res = get_request_by_id(&mut conn, user.id).await;
        assert!(res.is_ok());
        assert!(res.unwrap().code == "bruh".to_string());
        let res = delete_request(&mut conn, user.id).await;
        assert!(res.is_ok());
        delete_mock_user(user.id).await;
    }
}

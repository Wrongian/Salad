#[cfg(test)]
pub mod tests {
    use chrono::Utc;

    use crate::{
        connectors::db::{
            insight::{get_user_insights, update_user_insights},
            mock_connection,
        },
        models::insights::UpdateUserInsight,
        tests::{create_mock_user, delete_mock_user},
    };

    #[tokio::test]
    pub async fn it_can_update_user_insights() {
        let user = create_mock_user().await;

        let same_naive_date_time = Utc::now().naive_utc();

        let insert_insight = UpdateUserInsight {
            user_id: user.id,
            view_count: Some(1),
            follow_count: None,
            unfollow_count: None,
            follow_request_count: None,
            share_count: None,
            created_at: same_naive_date_time.clone(),
        };

        let mut conn = mock_connection().await;

        let result = update_user_insights(&mut conn, insert_insight).await;
        println!("result: {:?}", result);
        assert!(
            result.is_ok(),
            "Failed to insert user insights in connector call"
        );

        let update_insight_with_same_user_id = UpdateUserInsight {
            user_id: user.id,
            view_count: Some(1),
            follow_count: None,
            unfollow_count: None,
            follow_request_count: None,
            share_count: None,
            created_at: same_naive_date_time.clone(),
        };

        let result_again = update_user_insights(&mut conn, update_insight_with_same_user_id).await;
        assert!(
            result_again.is_ok(),
            "Failed to properly update user insights in connector call"
        );

        let field_validation = get_user_insights(&mut conn, user.id).await;

        assert!(
            field_validation.is_ok(),
            "Failed to get user insights in connector call"
        );
        let payload = field_validation.unwrap();
        assert_eq!(
            payload.len(),
            1,
            "Retrieved payload length is not singular."
        );
        assert_eq!(
            payload[0].view_count, 2,
            "View count did not update correctly."
        );

        // clean up
        delete_mock_user(user.id).await;
    }
}

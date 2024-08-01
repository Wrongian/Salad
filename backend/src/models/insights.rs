use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::user_insights)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetUserInsight {
    pub user_id: i32,
    pub view_count: i32,
    pub follow_count: i32,
    pub unfollow_count: i32,
    pub follow_request_count: i32,
    pub share_count: i32,
    pub created_bucket: NaiveDateTime,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = crate::schema::user_insights)]
pub struct UpdateUserInsight {
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub view_count: Option<i32>,
    pub follow_count: Option<i32>,
    pub unfollow_count: Option<i32>,
    pub follow_request_count: Option<i32>,
    pub share_count: Option<i32>,
}

pub trait Increment<R> {
    fn increment_view_count(user_id: i32, created_at: NaiveDateTime) -> R;
    fn increment_follow_count(user_id: i32, created_at: NaiveDateTime) -> R;
    fn increment_unfollow_count(user_id: i32, created_at: NaiveDateTime) -> R;
    fn increment_follow_request_count(user_id: i32, created_at: NaiveDateTime) -> R;
    fn increment_share_count(user_id: i32, created_at: NaiveDateTime) -> R;
}

impl Increment<UpdateUserInsight> for UpdateUserInsight {
    fn increment_view_count(user_id: i32, created_at: NaiveDateTime) -> UpdateUserInsight {
        UpdateUserInsight {
            user_id,
            created_at,
            view_count: Some(1),
            follow_count: None,
            unfollow_count: None,
            follow_request_count: None,
            share_count: None,
        }
    }

    fn increment_follow_count(user_id: i32, created_at: NaiveDateTime) -> UpdateUserInsight {
        UpdateUserInsight {
            user_id,
            created_at,
            view_count: None,
            follow_count: Some(1),
            unfollow_count: None,
            follow_request_count: None,
            share_count: None,
        }
    }

    fn increment_unfollow_count(user_id: i32, created_at: NaiveDateTime) -> UpdateUserInsight {
        UpdateUserInsight {
            user_id,
            created_at,
            view_count: None,
            follow_count: None,
            unfollow_count: Some(1),
            follow_request_count: None,
            share_count: None,
        }
    }

    fn increment_follow_request_count(
        user_id: i32,
        created_at: NaiveDateTime,
    ) -> UpdateUserInsight {
        UpdateUserInsight {
            user_id,
            created_at,
            view_count: None,
            follow_count: None,
            unfollow_count: None,
            follow_request_count: Some(1),
            share_count: None,
        }
    }

    fn increment_share_count(user_id: i32, created_at: NaiveDateTime) -> UpdateUserInsight {
        UpdateUserInsight {
            user_id,
            created_at,
            view_count: None,
            follow_count: None,
            unfollow_count: None,
            follow_request_count: None,
            share_count: Some(1),
        }
    }
}

use std::sync::Arc;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tide::Request;
use validator::Validate;

use crate::{
    connectors::db::insight::get_user_insights,
    helpers::{
        auth::get_session_user_id, state::get_connection, validation::validate_query_params,
    },
    models::insights::GetUserInsight,
    types::{response::Response, state::TideState},
};

#[derive(Deserialize, Validate)]
pub struct GetInsightQueryParams {}

#[derive(Serialize)]
pub struct GetInsightResponsePayload {
    total_profile_views: i32,
    interval_views: Vec<(NaiveDateTime, i32)>,
    interval_follows: Vec<(NaiveDateTime, i32)>,
    interval_unfollows: Vec<(NaiveDateTime, i32)>,
    interval_follow_requests: Vec<(NaiveDateTime, i32)>,
    interval_shares: Vec<(NaiveDateTime, i32)>,
}

impl GetInsightResponsePayload {
    fn from_get_user_insights(mut user_insights: Vec<GetUserInsight>) -> GetInsightResponsePayload {
        // sort ascending
        user_insights.sort_by(|a, b| a.created_bucket.cmp(&b.created_bucket));

        let total_profile_views = user_insights
            .iter()
            .map(|insight| insight.view_count)
            .reduce(|a, b| a + b)
            .unwrap_or(0);

        let interval_views = user_insights
            .iter()
            .map(|insight| (insight.created_bucket, insight.view_count))
            .collect::<Vec<(NaiveDateTime, i32)>>();

        let interval_follows = user_insights
            .iter()
            .map(|insight| (insight.created_bucket, insight.follow_count))
            .collect::<Vec<(NaiveDateTime, i32)>>();

        let interval_unfollows = user_insights
            .iter()
            .map(|insight| (insight.created_bucket, insight.unfollow_count))
            .collect::<Vec<(NaiveDateTime, i32)>>();

        let interval_follow_requests = user_insights
            .iter()
            .map(|insight| (insight.created_bucket, insight.follow_request_count))
            .collect::<Vec<(NaiveDateTime, i32)>>();

        let interval_shares = user_insights
            .iter()
            .map(|insight| (insight.created_bucket, insight.share_count))
            .collect::<Vec<(NaiveDateTime, i32)>>();

        GetInsightResponsePayload {
            total_profile_views,
            interval_views,
            interval_follows,
            interval_unfollows,
            interval_shares,
            interval_follow_requests,
        }
    }
}

pub async fn get_insights(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // validate query params
    let _ = match validate_query_params::<GetInsightQueryParams>(&req) {
        Ok(queries) => queries,
        Err(e) => return e.into_response(),
    };

    let mut conn = &mut get_connection(&mut req);

    let payload = match get_user_insights(&mut conn, user_id).await {
        Ok(user_insights) => GetInsightResponsePayload::from_get_user_insights(user_insights),
        Err(e) => return e.into_response(),
    };

    Response::new(payload).into_response()
}

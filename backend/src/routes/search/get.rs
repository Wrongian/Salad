use std::sync::Arc;

use serde::Deserialize;
use tide::Request;
use validator::Validate;

use crate::{
    connectors::db::user::{get_queried_user_total_count, get_queried_users},
    helpers::{state::get_connection, validation::validate_query_params},
    types::{
        error::Error,
        pagination::{GetPaginatedProfile, PaginatedGetPayload, PER_PAGE},
        response::Response,
        state::TideState,
    },
};

#[derive(Deserialize, Validate)]
struct SearchUserQueryParams {
    query: String,
    #[validate(range(min = 1))]
    index: i64,
}

pub async fn search_users(mut req: Request<Arc<TideState>>) -> tide::Result {
    let SearchUserQueryParams { query, index } = match validate_query_params(&req) {
        Ok(params) => params,
        Err(e) => return e.into_response(),
    };

    let mut conn = get_connection(&mut req);

    let profiles = match get_queried_users(&mut conn, query.clone(), index, PER_PAGE).await {
        Ok(user_tuples) => user_tuples
            .into_iter()
            .map(|user| GetPaginatedProfile {
                username: user.0.username,
                img_src: user.1.map(|img| img.img_src),
                id: user.0.id,
                display_name: user.0.display_name,
            })
            .collect::<Vec<GetPaginatedProfile>>(),
        Err(e) => return Error::DieselError(e).into_response(),
    };

    let total_size = match get_queried_user_total_count(&mut conn, query).await {
        Ok(total_profile_count) => total_profile_count,
        Err(e) => return Error::DieselError(e).into_response(),
    };

    Response::new(PaginatedGetPayload {
        profiles,
        total_size,
    })
    .into_response()
}

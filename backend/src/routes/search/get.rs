use std::sync::Arc;

use serde::Deserialize;
use tide::Request;

use crate::{
    connectors::db::user::get_queried_users,
    helpers::{auth::get_session_user_id, state::get_connection},
    types::{
        error::{Error, RequestErrors},
        pagination::{GetPaginatedProfile, PaginatedGetPayload, PER_PAGE},
        response::Response,
        state::TideState,
    },
};

#[derive(Deserialize)]
struct SearchUserQueryParams {
    query: String,
    index: i64,
}

impl SearchUserQueryParams {
    fn get_filter_object(&self) {}
}

// session is not needed (for now).
pub async fn search_users(mut req: Request<Arc<TideState>>) -> tide::Result {
    let SearchUserQueryParams { query, index } = match req.query::<SearchUserQueryParams>() {
        Ok(params) => params,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedParams).into_response();
        }
    };

    let mut conn = get_connection(&mut req);
    let profiles = match get_queried_users(&mut conn, query, index, PER_PAGE).await {
        Ok(p) => p
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

    Response::new(PaginatedGetPayload { profiles }).into_response()
}

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tide::Request;

use crate::{
    connectors::db::follow::{
        get_queried_follower_total_count, get_queried_followers, get_queried_following_total_count,
        get_queried_followings, has_follow_request, is_following,
    },
    helpers::{auth::get_session_user_id, state::get_connection},
    types::{
        error::{Error, RequestErrors},
        pagination::{GetPaginatedProfile, PaginatedGetPayload, PER_PAGE},
        response::Response,
        state::TideState,
    },
};

#[derive(Deserialize)]
struct FollowStatusParams {
    pub id: i32,
}
#[derive(Serialize)]
struct FollowStatusResponsePayload {
    status: String,
}

impl FollowStatusResponsePayload {
    pub fn following() -> FollowStatusResponsePayload {
        FollowStatusResponsePayload {
            status: String::from("following"),
        }
    }

    pub fn none() -> FollowStatusResponsePayload {
        FollowStatusResponsePayload {
            status: String::from("none"),
        }
    }

    pub fn pending() -> FollowStatusResponsePayload {
        FollowStatusResponsePayload {
            status: String::from("pending"),
        }
    }

    pub fn into_response(self) -> tide::Result {
        Response::new(self).into_response()
    }
}

#[derive(Deserialize)]
struct FollowGetQueryParams {
    query: String,
    index: i64,
}

pub async fn get_follow_status(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    let to_id = match req.query::<FollowStatusParams>() {
        Ok(params) => params.id,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedParams).into_response()
        }
    };
    let mut conn = get_connection(&mut req);
    // check user is following to_id
    match is_following(&mut conn, user_id, to_id).await {
        Ok(true) => return FollowStatusResponsePayload::following().into_response(),
        Ok(false) => (),
        Err(e) => return Error::DieselError(e).into_response(),
    }

    // check pending request
    match has_follow_request(&mut conn, to_id, user_id).await {
        Ok(true) => FollowStatusResponsePayload::pending().into_response(),
        Ok(false) => FollowStatusResponsePayload::none().into_response(),
        Err(e) => Error::DieselError(e).into_response(),
    }
}

pub async fn get_followers(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    let FollowGetQueryParams { query, index } = match req.query::<FollowGetQueryParams>() {
        Ok(params) => params,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedParams).into_response();
        }
    };

    let mut conn = get_connection(&mut req);

    // get brief profile information (name, profile picture, etc.) of followers
    let profiles =
        match get_queried_followers(&mut conn, query.clone(), user_id, index, PER_PAGE).await {
            Ok(result) => result
                .into_iter()
                .map(|user| GetPaginatedProfile {
                    display_name: user.0.display_name,
                    id: user.0.id,
                    img_src: user.1.map(|img| img.img_src),
                    username: user.0.username,
                })
                .collect::<Vec<GetPaginatedProfile>>(),
            Err(e) => return Error::DieselError(e).into_response(),
        };

    let total_size = match get_queried_follower_total_count(&mut conn, user_id, query).await {
        Ok(total_following_count) => total_following_count,
        Err(e) => return Error::DieselError(e).into_response(),
    };
    Response::new(PaginatedGetPayload {
        profiles,
        total_size,
    })
    .into_response()
}

pub async fn get_following(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    let FollowGetQueryParams { query, index } = match req.query::<FollowGetQueryParams>() {
        Ok(params) => params,
        Err(_) => {
            return Error::InvalidRequestError(RequestErrors::MalformedParams).into_response();
        }
    };

    let mut conn = get_connection(&mut req);

    // get brief profile information (name, profile picture, etc.) of users being followed
    let profiles =
        match get_queried_followings(&mut conn, query.clone(), user_id, index, PER_PAGE).await {
            Ok(result) => result
                .into_iter()
                .map(|user| GetPaginatedProfile {
                    display_name: user.0.display_name,
                    id: user.0.id,
                    img_src: user.1.map(|img| img.img_src),
                    username: user.0.username,
                })
                .collect::<Vec<GetPaginatedProfile>>(),
            Err(e) => return Error::DieselError(e).into_response(),
        };

    let total_size = match get_queried_following_total_count(&mut conn, user_id, query).await {
        Ok(total_following_count) => total_following_count,
        Err(e) => return Error::DieselError(e).into_response(),
    };

    Response::new(PaginatedGetPayload {
        profiles,
        total_size,
    })
    .into_response()
}

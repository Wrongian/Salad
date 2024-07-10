use std::sync::Arc;

use serde::Serialize;
use tide::{log::error, Request};

use crate::{
    connectors::db::{
        link::get_user_links_by_id,
        user::{check_user_exists, check_username_present, get_user_profile_by_username},
    },
    helpers::{auth::get_session_username, links::linearise, params::extract_username_from_params},
    types::{error::Error, response::Response, state::TideState},
};

#[derive(Debug, Serialize)]
struct GetLinksResponseBody {
    links: Vec<GetImagedLink>,
}

#[derive(Debug, Serialize, Clone)]
pub struct GetImagedLink {
    pub id: i32,
    pub user_id: i32,
    pub next_id: Option<i32>,
    pub description: Option<String>,
    pub title: Option<String>,
    pub href: String,
    pub img_src: Option<String>,
}

pub async fn get_links(req: Request<Arc<TideState>>) -> tide::Result {
    // get session username from session or default to ""
    let session_username = get_session_username(&req).unwrap_or("".to_string());

    // get username from params
    let username = match extract_username_from_params(&req) {
        Ok(usr) => usr,
        Err(err) => return err.into_response(),
    };

    let is_owner = session_username == username;

    // get connection state
    let state = req.state();
    let mut conn = state.tide_pool.get().unwrap();

    // check username exists
    if !check_username_present(&mut conn, &username).await {
        return Error::NotFoundError(String::from("User")).into_response();
    }

    // check if profile is private, if it is then return empty vec
    let profile = match get_user_profile_by_username(&mut conn, &username).await {
        Ok(data) => data,
        Err(e) => return Error::DieselError(e).into_response(),
    };

    if !is_owner && profile.is_private {
        // if origin is not owner and querying a private profile, return empty links
        return Response::new(GetLinksResponseBody { links: Vec::new() }).into_response();
    }
    // otherwise either owner or querying a public profile.
    // Thus, get all links and return
    match get_user_links_by_id(&mut conn, profile.id).await {
        Ok(links) => Response::new(GetLinksResponseBody {
            links: linearise(
                &links
                    .into_iter()
                    .map(|link| GetImagedLink {
                        id: link.0.id,
                        user_id: link.0.user_id,
                        next_id: link.0.next_id,
                        description: link.0.description,
                        title: link.0.title,
                        href: link.0.href,
                        img_src: link.1.map(|img| img.img_src),
                    })
                    .collect::<Vec<GetImagedLink>>(),
            ),
        })
        .into_response(),
        Err(e) => {
            error!("Error in retrieving user links by id: {}", e);
            Error::DieselError(e).into_response()
        }
    }
}

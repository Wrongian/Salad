use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tide::{log::error, Request};
use validator::Validate;

use crate::{
    connectors::db::{self, connection::DBConnection},
    helpers::auth::get_session_user_id,
    models::links::InsertLink,
    types::{
        error::{Error, RequestErrors},
        response::Response,
        state::TideState,
    },
};

#[derive(Deserialize, Serialize, Validate)]
struct CreateLinkParams {
    title: Option<String>,
    bio: Option<String>,
    href: String,
}

// POST end point for adding a link
pub async fn add_link(mut req: Request<Arc<TideState>>) -> tide::Result {
    // extract user id from session
    let user_id = match get_session_user_id(&req) {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };
    // get payload
    let link_params: CreateLinkParams;
    match req.body_json().await {
        Ok(params) => {
            link_params = params;
        }
        Err(e) => {
            error!("Error occurred in parsing: {:?}", e);
            return Error::InvalidRequestError(RequestErrors::MalformedPayload).into_response();
        }
    }

    // validate payload
    match link_params.validate() {
        Err(e) => return Error::ValidationError(e).into_response(),
        _ => (),
    }

    let state = req.state();
    let mut conn: DBConnection = state.tide_pool.get().unwrap();

    // add to database
    let insert_link = InsertLink {
        user_id,
        next_id: None,
        description: link_params.bio,
        title: link_params.title,
        href: link_params.href,
    };

    match db::link::create(&mut conn, &insert_link).await {
        Ok(_) => Response::empty().into_response(),
        Err(e) => {
            // failed to create link
            error!("Error creating link. {:?}, Error: {}", insert_link, e);
            return Error::DieselError(e).into_response();
        }
    }
}

use std::sync::Arc;

use crate::response::error::Error;
use tide::Request;

use crate::TideState;

pub fn get_session_user_id(req: &Request<Arc<TideState>>) -> Result<i32, tide::Result> {
    req.session()
        .get::<i32>("user_id")
        .ok_or_else(|| Error::InvalidSessionError().into_response())
}

pub fn get_session_username(req: &Request<Arc<TideState>>) -> Result<String, tide::Result> {
    req.session()
        .get("username")
        .ok_or_else(|| Error::InvalidSessionError().into_response())
}

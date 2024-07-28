use crate::types::{error::Error, state::TideState};
use std::sync::Arc;
use tide::Request;

// functions related to authentication

pub fn get_session_user_id(req: &Request<Arc<TideState>>) -> Result<i32, Error> {
    req.session()
        .get::<i32>("user_id")
        .ok_or_else(|| Error::InvalidSessionError())
}

pub fn get_session_username(req: &Request<Arc<TideState>>) -> Result<String, Error> {
    req.session()
        .get("username")
        .ok_or_else(|| Error::InvalidSessionError())
}

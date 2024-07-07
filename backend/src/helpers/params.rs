use crate::types::{
    error::{Error, RequestErrors},
    state::TideState,
};
use std::sync::Arc;
use tide::Request;

// various functions for extracting params

pub fn extract_link_id_from_params(req: &Request<Arc<TideState>>) -> Result<i32, Error> {
    req.param("link_id")
        .map_err(|_| ())
        .and_then(|link| link.parse().map_err(|_| ()))
        .map_err(|_| Error::InvalidRequestError(RequestErrors::MalformedParams))
}

pub fn extract_username_from_params(req: &Request<Arc<TideState>>) -> Result<String, Error> {
    req.param("username")
        .map(|username| username.to_string())
        .map_err(|_| Error::InvalidRequestError(RequestErrors::MalformedParams))
}

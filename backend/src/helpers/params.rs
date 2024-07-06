use crate::{response::error::Error, TideState};
use std::sync::Arc;
use tide::Request;

pub fn extract_link_id_from_params(req: &Request<Arc<TideState>>) -> Result<i32, tide::Result> {
    req.param("link_id")
        .map_err(|_| ())
        .and_then(|link| link.parse().map_err(|_| ()))
        .map_err(|_| Error::InvalidRequestError().into_response())
}

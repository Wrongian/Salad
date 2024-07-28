use std::sync::Arc;

use serde::Deserialize;
use tide::Request;
use validator::Validate;

use crate::types::{
    error::{Error, RequestErrors},
    state::TideState,
};

pub fn validate_query_params<Q: for<'a> Deserialize<'a> + Validate>(
    req: &Request<Arc<TideState>>,
) -> Result<Q, Error> {
    req.query::<Q>()
        .map_err(|_| Error::InvalidRequestError(RequestErrors::MalformedParams))
        .and_then(|params| {
            params
                .validate()
                .map_err(|e| Error::ValidationError(e))
                .map(|_| params)
        })
}

use serde::Serialize;
use tide::StatusCode;

use super::error::Error;

pub struct Response<T: Serialize> {
    payload: T,
}

impl<T: Serialize> Response<T> {
    pub fn into_response(self) -> tide::Result {
        tide::Body::from_json(&self.payload)
            .map(|body| tide::Response::builder(StatusCode::Ok).body(body).build())
            .or_else(|_| Error::InvalidResponseError().into_response())
    }
}

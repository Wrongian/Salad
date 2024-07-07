use super::error::Error;
use serde::Serialize;
use tide::StatusCode;

// this a custom response struct that abstracts a tide response
pub struct Response<T: Serialize = EmptyBody> {
    pub payload: T,
}

impl Response {
    pub fn empty() -> Response<EmptyBody> {
        Response {
            payload: EmptyBody {},
        }
    }
}

impl<T: Serialize> Response<T> {
    pub fn new(payload: T) -> Response<T> {
        Response { payload }
    }
    pub fn into_response(self) -> tide::Result {
        tide::Body::from_json(&self.payload)
            .map(|body| tide::Response::builder(StatusCode::Ok).body(body).build())
            .or_else(|_| Error::InvalidResponseError().into_response())
    }
}

#[derive(Serialize)]
pub struct EmptyBody {}

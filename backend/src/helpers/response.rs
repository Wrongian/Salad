use serde::Serialize;
use tide::Response;

// standard response body without error
#[derive(Debug, Serialize)]
pub struct StandardBody {
    pub result: bool,
    pub err: String,
}
// build a tide result with standard response body
pub fn build_standard_response(result: bool, err: String, status: u16) -> tide::Result {
    // build response
    let res_body = StandardBody { result, err };
    let response = Response::builder(status)
        .body(tide::Body::from_json(&res_body)?)
        .build();
    Ok(response)
}
// build the standard response
pub fn build_response(body: impl serde::Serialize, status: u16) -> tide::Result {
    // build response
    let response = Response::builder(status)
        .body(tide::Body::from_json(&body)?)
        .build();
    Ok(response)
}

// build an error response
pub fn build_error(message: String, status: u16) -> tide::Result {
    let err_body = StandardBody {
        result: false,
        err: message,
    };
    let response = Response::builder(status)
        .body(tide::Body::from_json(&err_body)?)
        .build();
    Ok(response)
}

pub fn build_success() -> tide::Result {
    build_standard_response(true, "".to_string(), 200)
}

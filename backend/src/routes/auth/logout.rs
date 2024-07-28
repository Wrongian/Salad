use std::sync::Arc;

use tide::Request;

use crate::types::response::Response;
use crate::types::state::TideState;

// get route that logs the user out from the website
pub async fn logout(mut req: Request<Arc<TideState>>) -> tide::Result {
    let session = req.session_mut();
    session.destroy();
    return Response::empty().into_response();
}

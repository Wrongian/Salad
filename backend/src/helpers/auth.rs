use std::sync::Arc;

use tide::Request;

use crate::TideState;

pub fn get_session_user_id(req: &Request<Arc<TideState>>) -> Result<i32, tide::Error> {
    req.session()
        .get::<i32>("user_id")
        .ok_or_else(|| tide::Error::from_str(400, "Invalid session!"))
}

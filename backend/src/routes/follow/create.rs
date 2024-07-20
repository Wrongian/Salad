use std::sync::Arc;

use tide::Request;

use crate::types::state::TideState;

pub async fn create_outbound_follow_request(mut req: Request<Arc<TideState>>) {

    // check if user with to_id exists

    // check to_id != user_id

    // check if request record already exists, or already following

    // add new record to db

    // TODO: publish notification
}

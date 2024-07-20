use std::sync::Arc;

use tide::Request;

use crate::types::state::TideState;

pub async fn settle_inbound_follow_request(mut req: Request<Arc<TideState>>) {

    // check if pending follow request record exists

    // check accept field: true | false

    // delete the record from pending_follow_requests

    // if accept, then add to followers; otherwise do nothing

    // TODO: publish notification
}

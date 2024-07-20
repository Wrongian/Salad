use std::sync::Arc;

use tide::Request;

use crate::types::state::TideState;

pub async fn delete_inbound_follow_request(mut req: Request<Arc<TideState>>) {

    // check if request with the pending_follower_id exists

    // delete record from db (pending_follow_requests table)

    // TODO: delete tagged notifications
}

pub async fn delete_outbound_follow_request(mut req: Request<Arc<TideState>>) {

    // check if request with the pending_follow_id exists

    // delete record from db (pending_follow_requests table)

    // TODO: delete tagged notifications
}

pub async fn delete_follower(mut req: Request<Arc<TideState>>) {

    // check if user with follower_id is a follower of user_id

    // delete record
}

pub async fn delete_following(mut req: Request<Arc<TideState>>) {

    // check if user with user_id follows following_id

    // delete record
}

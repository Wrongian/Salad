use std::sync::Arc;

use tide::Request;

use crate::types::state::TideState;

pub async fn handle_follow_request(mut req: Request<Arc<TideState>>) {}

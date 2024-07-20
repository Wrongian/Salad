use std::sync::Arc;

use tide::Request;

use crate::{connectors::db::connection::DBConnection, types::state::TideState};

pub fn get_connection(req: &mut Request<Arc<TideState>>) -> DBConnection {
    req.state().tide_pool.get().unwrap()
}

pub fn get_s3_client(req: &mut Request<Arc<TideState>>) -> &aws_sdk_s3::Client {
    &req.state().s3_client
}

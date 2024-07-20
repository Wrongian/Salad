use aws_sdk_s3::{self as s3};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::path::Path;
use tempfile::TempDir;

// this defines the state of the tide app

// this is the pooled connection
pub type TidePool = Pool<ConnectionManager<PgConnection>>;

// this is actual state of the tide app as a struct
pub struct TideState {
    pub tide_pool: TidePool,
    pub s3_client: s3::Client,
    pub tempdir: TempDir,
}

// this returns the path of the directory
impl TideState {
    fn path(&self) -> &Path {
        self.tempdir.path()
    }
}

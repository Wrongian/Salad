use crate::connectors::smtp::email::EmailService;
use crate::connectors::smtp::smtp_service::SMTPService;
use aws_sdk_s3::{self as s3};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::path::Path;
use tempfile::TempDir;

// this defines the state of the tide app

// this is the pooled connection
pub type TidePool = Pool<ConnectionManager<PgConnection>>;

// this is actual state of the tide app as a struct
pub struct TideState<T: SMTPService = EmailService> {
    pub tide_pool: TidePool,
    pub s3_client: s3::Client,
    pub tempdir: TempDir,
    // might want to make this a dynamic type in the future
    // or make this generic, tried making it generic but broke everything because you have to change a million things
    pub email_service: T,
}

// this returns the path of the directory
impl<T: SMTPService> TideState<T> {
    fn path(&self) -> &Path {
        self.tempdir.path()
    }
}

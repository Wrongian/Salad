// defines the smtp trait which is used as an interface

use crate::types::error::Error;

pub trait SMTPService {
    fn send_email(&self, to_email: String, subject: String, body: String) -> Result<(), Error>;
}

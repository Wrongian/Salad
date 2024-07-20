// defines the smtp trait which is used as an interface

use crate::types::error::Error;

trait SMTPService {
    async fn send_email(to_email: String) -> Result<(), Error>;
}

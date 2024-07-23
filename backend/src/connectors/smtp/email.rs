use crate::connectors::smtp::smtp_service::SMTPService;
use crate::types::error::Error;
use crate::types::error::Error::{AddressError, EmailError};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub struct EmailService {
    // I shall avoid storing the credentials in the struct
    email_host: String,
}

impl EmailService {
    pub fn new() -> EmailService {
        let host = env::var("SMTP_HOST").expect("SMTP host not found in .env");
        return EmailService { email_host: host };
    }

    fn get_credentials() -> Credentials {
        // honestly this should return a custom error
        let username = env::var("SMTP_USERNAME").expect("SMTP username not found in .env");
        let password = env::var("SMTP_PASSWORD").expect("SMTP password not found in .env");
        // consume both
        let cred = Credentials::new(username, password);
        return cred;
    }
}

impl SMTPService for EmailService {
    fn send_email(&self, to_email: String, subject: String, body: String) -> Result<(), Error> {
        // credentials
        let cred = EmailService::get_credentials();
        // we open a new connection each time
        // not sure if putting the connection in the app will be super reliable
        let conn = SmtpTransport::relay(&self.email_host)
            .unwrap()
            .credentials(cred)
            .build();
        let username = env::var("SMTP_USERNAME").expect("SMTP username not found in .env");
        let to_address: Mailbox = match to_email.parse() {
            Ok(address) => address,
            Err(e) => return Err(AddressError(e)),
        };
        let from_address: Mailbox = match username.parse() {
            Ok(address) => address,
            Err(e) => return Err(AddressError(e)),
        };
        // make message
        let message = Message::builder()
            .from(from_address)
            .to(to_address)
            .subject(subject)
            .body(body)
            .unwrap();

        // send the email
        match conn.send(&message) {
            Ok(_) => {}
            Err(e) => return Err(EmailError(e)),
        }
        Ok(())
    }
}

// testing
#[cfg(test)]
mod email_tests {
    use crate::connectors::smtp::email::EmailService;
    use crate::connectors::smtp::smtp_service::SMTPService;
    use dotenvy::dotenv;
    use std::env;
    #[test]
    pub fn test_send_email() {
        dotenv().expect("No .env file found");
        let email_service = EmailService::new();

        let username = env::var("SMTP_USERNAME").expect("SMTP username not found in .env");
        let _res = email_service
            .send_email(
                username,
                "testing".to_string(),
                "testing sending email".to_string(),
            )
            .expect("email not sent");
    }
}

use crate::types::Error::EmailError;
use crate::types::Error::Error;
use dotenvy::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

struct EmailService {
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
        let conn = SmtpTransport::relay(self.host)
            .unwrap()
            .credentials(&cred)
            .build();

        let username = env::var("SMTP_USERNAME").expect("SMTP username not found in .env");
        // make message
        let message = Message::builder()
            .from(username)
            .to(to_email)
            .subject(subject)
            .body(body)
            .unwrap();

        // send the email
        match conn.send(message) {
            Ok(_) => {}
            Err(e) => return EmailError(),
        }
        Ok()
    }
}

use crate::types::Error::Error;
use dotenvy::dotenv;
use lettre::transport::smtp::authentication::Credentials;

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
    fn send_email(to_email: String) -> Result<(), Error> {
        // stub
        Ok()
    }
}

use crate::types::Error::Error;
use dotenvy::dotenv;
use lettre::transport::smtp::authentication::Credentials;

struct EmailService {
    // I shall avoid storing the credentials in the struct
    email_host: String,
}

impl EmailService {
    pub fn new() -> EmailService {}

    fn get_credentials() -> Credentials {
        let username = env::var("SMTP_USERNAME").expect("SMTP Username not Found in .env");
        let password = env::var("SMTP_PASSWORD").expect("SMTP Password not Found in .env");
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

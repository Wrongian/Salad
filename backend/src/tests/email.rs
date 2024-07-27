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

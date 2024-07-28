use lettre::message::{header, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use std::env;

use super::current_time;

pub async fn send_mail(mail: &str, thread_id: &str) {
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
    let reply_to = env::var("REPLY_TO_MAIL").expect("REPLY_TO_MAIL must be set");

    let email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .reply_to(reply_to.parse().unwrap())
        .to(mail.parse().unwrap())
        .subject("Hello from xChange!")
        .header(header::ContentType::TEXT_PLAIN)
        .body(String::from(
            "This is a test email from xChange using lettre.",
        ))
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_pass);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!(
            "{}: Worker {} Email sent successfully!",
            current_time(),
            thread_id
        ),
        Err(e) => println!("Could not send email: {:?}", e),
    }
}

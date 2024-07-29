use lettre::message::{header, Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use std::env;
use totp_rs::{Algorithm, TOTP};

use crate::constants::get_email_html;

use super::current_time;

pub async fn send_mail(mail: &str, otp: &str, thread_id: &str) {
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
    let reply_to = env::var("REPLY_TO_MAIL").expect("REPLY_TO_MAIL must be set");

    let email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .reply_to(reply_to.parse().unwrap())
        .to(mail.parse().unwrap())
        .subject("[verify]: Email verification otp from xChange!")
        .multipart(
            MultiPart::alternative().singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(get_email_html(otp)),
            ),
        )
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_pass);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!(
            "{}: Worker {}: Email sent successfully!",
            current_time(),
            thread_id
        ),
        Err(e) => println!("Could not send email: {:?}", e),
    }
}

pub fn generate_otp() -> String {
    let otp_secret = env::var("OTP_SECRET").expect("OTP_SECRET must be set");
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        2 * 60,
        otp_secret.as_bytes().to_vec(),
    )
    .unwrap();

    let otp = totp.generate_current().unwrap();
    otp
}

pub fn parse_json<T>(data: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(data).unwrap()
}

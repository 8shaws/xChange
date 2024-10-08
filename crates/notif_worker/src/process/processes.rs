use r2d2_redis::redis;
use serde_json;
use serde_json::Value;
use std::thread;
use tokio::time::{sleep, Duration};

use crate::utils::lib::generate_otp;
use crate::utils::lib::send_mail;
use common::utils::current_time;

pub async fn send_email_process(conn: &mut redis::Connection) {
    let thread_id = format!("{:?}", thread::current().id());
    let queue = "user_email_verify";

    let result = redis::cmd("RPOP").arg(queue).query::<Option<String>>(conn);

    match result {
        Ok(Some(mail)) => {
            println!(
                "{}: Worker {}: processing item from {}: {}",
                current_time(),
                thread_id,
                queue,
                mail
            );

            let otp = generate_otp();
            let otp_clone = otp.clone();

            let catch_otp_result = redis::cmd("SET")
                .arg(format!("otp:{}", mail))
                .arg(otp)
                .query::<Option<String>>(conn);

            match catch_otp_result {
                Ok(_) => {
                    println!(
                        "{}: Worker {}: OTP for {} saved successfully!",
                        current_time(),
                        thread_id,
                        mail
                    );

                    send_mail(&mail, &otp_clone, &thread_id).await;
                }
                Err(err) => {
                    println!(
                        "{}: Worker {}: Failed to save OTP for {}: {}",
                        current_time(),
                        thread_id,
                        mail,
                        err
                    );
                }
            }

            sleep(Duration::from_secs(1)).await;
        }
        Ok(None) => {
            println!(
                "{}: Worker {}: {} is empty after length check, waiting...",
                current_time(),
                thread_id,
                queue
            );
            sleep(Duration::from_secs(2)).await;
        }
        Err(err) => {
            println!(
                "{}: Worker {}: Failed to pop item from {}: {}",
                current_time(),
                thread_id,
                queue,
                err
            );
        }
    }
}

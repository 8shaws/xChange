use r2d2_redis::redis;
use std::thread;
use tokio::time::{sleep, Duration};

use crate::utils::current_time;
use crate::utils::lib::send_mail;

pub async fn send_email_process(conn: &mut redis::Connection) {
    let thread_id = format!("{:?}", thread::current().id());
    let queue = "user_email_verify";

    let result = redis::cmd("RPOP").arg(queue).query::<Option<String>>(conn);

    match result {
        Ok(Some(mail)) => {
            println!(
                "{}: Worker {} processing item from {}: {}",
                current_time(),
                thread_id,
                queue,
                mail
            );
            send_mail(&mail, &thread_id).await;
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

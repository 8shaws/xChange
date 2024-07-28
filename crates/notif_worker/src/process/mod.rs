use dotenvy::dotenv;
use std::sync::Arc;
use std::thread;
use tokio::time::{sleep, Duration};

mod processes;

use crate::utils::current_time;
use crate::utils::get_queue_with_max_length;
use crate::utils::RedisPool;

pub async fn handle_process(conn: Arc<RedisPool>, queues: Vec<&str>) {
    dotenv().ok();

    let thread_id = format!("{:?}", thread::current().id());
    loop {
        let con_result = conn.get();

        match con_result {
            Ok(mut conn) => {
                if let Some(max_queue) = get_queue_with_max_length(&mut conn, &queues) {
                    if max_queue == "user_email_verify" {
                        processes::send_email_process(&mut conn).await;
                    }
                } else {
                    println!(
                        "{}: Worker {}: All queues are empty, waiting...",
                        current_time(),
                        thread_id
                    );
                    sleep(Duration::from_secs(2)).await;
                }
            }
            Err(err) => {
                println!(
                    "{}: Worker {}: Failed to get Redis connection: {}",
                    current_time(),
                    thread_id,
                    err
                );
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}

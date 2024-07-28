use r2d2_redis::redis;
use std::sync::Arc;
use std::thread;
use tokio::time::{sleep, Duration};

use crate::utils::current_time;
use crate::utils::RedisPool;

pub async fn handle_process(conn: Arc<RedisPool>) {
    let thread_id = format!("{:?}", thread::current().id());
    loop {
        let con_result = conn.get();

        match con_result {
            Ok(mut conn) => {
                let result: redis::RedisResult<Option<String>> =
                    redis::cmd("PING").query(&mut *conn);

                match result {
                    Ok(Some(item)) => {
                        println!(
                            "{}: Worker {} processing item: {}",
                            current_time(),
                            thread_id,
                            item
                        );

                        sleep(Duration::from_secs(1)).await;
                    }
                    Ok(None) => {
                        println!(
                            "{}: Worker {}: Queue is empty, waiting...",
                            current_time(),
                            thread_id
                        );
                        sleep(Duration::from_secs(2)).await;
                    }
                    Err(err) => {
                        println!(
                            "{}: Worker {}: Failed to pop item: {}",
                            current_time(),
                            thread_id,
                            err
                        );
                        sleep(Duration::from_secs(2)).await;
                    }
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

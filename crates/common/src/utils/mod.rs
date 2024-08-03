use chrono;
use r2d2_redis::redis;
use std::collections::HashMap;

pub fn current_time() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn get_queue_with_max_length(conn: &mut redis::Connection, queues: &[&str]) -> Option<String> {
    let mut queue_lengths = HashMap::new();

    for queue in queues {
        let len_result: redis::RedisResult<usize> =
            redis::cmd("LLEN").arg(*queue).query(&mut *conn);
        match len_result {
            Ok(len) => {
                queue_lengths.insert(queue.to_string(), len);
            }
            Err(err) => {
                println!("Failed to get length of {}: {}", queue, err);
            }
        }
    }

    queue_lengths
        .into_iter()
        .max_by_key(|&(_, len)| len)
        .map(|(queue, _)| queue)
}

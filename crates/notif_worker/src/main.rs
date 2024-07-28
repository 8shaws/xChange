use dotenvy::dotenv;
use std::sync::Arc;
use tokio::signal;

mod process;
mod utils;

use process::handle_process;
use utils::initialize_redis_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let no_worker_threads = std::env::var("NO_WORKER_THREADS")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<usize>()
        .expect("NO_WORKER_THREADS must be a positive integer");

    let queues = vec!["user_email_verify"];

    let pool = Arc::new(initialize_redis_pool());

    let mut worker_handlers = vec![];

    for _ in 0..no_worker_threads {
        let pool_clone = Arc::clone(&pool);
        let queues_clone = queues.clone();
        let worker_handle = tokio::spawn(async move {
            handle_process(pool_clone, queues_clone).await;
        });
        worker_handlers.push(worker_handle);
    }

    let shutdown_signal = async {
        signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        println!("{}: Shutdown signal received", utils::current_time());
    };

    shutdown_signal.await;

    for handle in worker_handlers {
        handle.abort();
    }
    println!("{}: Shutting down...", utils::current_time());
}

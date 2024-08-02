use common::types::order::{NewOrder, OrderResponse};
use futures::future::FutureExt;
use r2d2_redis::{
    r2d2,
    redis::{Commands, ErrorKind, RedisError},
    RedisConnectionManager,
};
use serde_json::{json, Value};
use std::env;
use tokio::sync::oneshot;

pub type RedisPool = r2d2::Pool<RedisConnectionManager>;

pub fn initialize_redis_pool() -> RedisPool {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(redis_url).unwrap();
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

struct RedisClient;

impl RedisClient {
    fn place_order_and_wait(
        pool: &RedisPool,
        order: NewOrder,
    ) -> impl futures::Future<Output = Result<OrderResponse, RedisError>> {
        let (tx, rx) = oneshot::channel();
        let client_order_id = uuid::Uuid::new_v4().to_string();
        let mut order = order;
        order.client_order_id = Some(client_order_id.clone());
        let order_json = serde_json::to_string(&order).unwrap();

        let pool_clone = pool.clone();

        tokio::spawn(async move {
            let mut conn = pool_clone.get().expect("Failed to get connection");
            let mut pubsub = conn.as_pubsub();
            pubsub.subscribe(&client_order_id).unwrap();

            let mut conn_clone = pool_clone.get().expect("Failed to get connection");
            let _: () = conn_clone
                .lpush(format!("orders@{}", order.symbol), order_json)
                .expect("Failed to push order to queue");

            match pubsub.get_message() {
                Ok(msg) => {
                    let payload: String = msg.get_payload().unwrap();
                    let payload_json: OrderResponse = serde_json::from_str(&payload).unwrap();
                    pubsub.unsubscribe(&client_order_id).unwrap();
                    tx.send(payload_json).unwrap();
                }
                Err(e) => {
                    eprintln!("Error receiving message: {:?}", e);
                }
            }
        });

        rx.map(|result| {
            result.map_err(|e| {
                RedisError::from((ErrorKind::IoError, "Oneshot channel error", e.to_string()))
            })
        })
        .boxed()
    }
}

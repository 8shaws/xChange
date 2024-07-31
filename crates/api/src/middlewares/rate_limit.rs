use actix_service::Service;
use actix_web::{web, App, Error, HttpRequest, HttpResponse};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::Semaphore;

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    requests: Arc<Mutex<HashMap<String, usize>>>,
    max_requests: usize,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_duration: Duration) -> Self {
        RateLimiter {
            semaphore: Arc::new(Semaphore::new(max_requests)),
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_duration,
        }
    }

    pub async fn check_rate_limit(&self, key: &str) -> Result<(), HttpResponse> {
        let mut requests = self.requests.lock().unwrap();
        let count = requests.entry(key.to_string()).or_insert(0);
        if *count >= self.max_requests {
            Err(HttpResponse::TooManyRequests().finish())
        } else {
            *count += 1;
            Ok(())
        }
    }

    pub async fn handle_request(
        &self,
        key: &str,
        request: HttpRequest,
    ) -> Result<HttpResponse, Error> {
        self.check_rate_limit(key).await;
        Ok(HttpResponse::Ok().finish())
    }
}

async fn rate_limited_handler(
    req: HttpRequest,
    rate_limiter: web::Data<RateLimiter>,
) -> Result<HttpResponse, Error> {
    let key = req.peer_addr().unwrap().to_string();
    rate_limiter.handle_request(&key, req).await
}

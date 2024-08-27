extern crate r2d2;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use env_logger::Builder;
use lazy_static::lazy_static;
use num_cpus;
use route::order::order_config;
use serde_json::json;
use std::time::{Duration, SystemTime};

use db::fns::initialize_db_pool;
mod auth;
mod kafka;
mod middlewares;
mod redis;
mod route;
mod types;

use crate::middlewares::rate_limit::RateLimiter;
use crate::route::user::user_config;

lazy_static! {
    static ref START_TIME: SystemTime = SystemTime::now();
}

async fn root() -> impl Responder {
    let uptime = match SystemTime::now().duration_since(*START_TIME) {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    };
    HttpResponse::Ok().json(json!({
        "uptime": uptime,
        "status": "Api server is healthy!"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let thread_count = num_cpus::get() - 6;

    Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let dp_pool = initialize_db_pool();
    let redis_pool = redis::initialize_redis_pool();

    let app_state = web::Data::new(types::AppState {
        db_pool: dp_pool,
        redis_pool: redis_pool,
    });

    let rate_limiter = web::Data::new(RateLimiter::new(10, Duration::from_secs(60)));

    println!("{:?}: Api Server is running on port: {}", *START_TIME, 8080);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .configure(|cfg| user_config(cfg, app_state.clone(), rate_limiter.clone()))
            .configure(|cfg| order_config(cfg, app_state.clone()))
            .route("/", web::get().to(root))
            .route("/_health", web::get().to(root))
    })
    .workers(thread_count)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

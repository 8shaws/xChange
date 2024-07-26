use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use db::initialize_db_pool;
use dotenvy::dotenv;
use env_logger::Builder;
use lazy_static::lazy_static;
use num_cpus;
use serde_json::json;
use std::time::SystemTime;

mod auth;
mod db;
mod models;
mod route;
mod schema;

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

    let pool = initialize_db_pool();

    println!("{:?}: Api Server is running on port: {}", *START_TIME, 8080);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(user_config)
            .route("/", web::get().to(root))
            .route("/_health", web::get().to(root))
    })
    .workers(thread_count)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

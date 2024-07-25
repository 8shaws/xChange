use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use env_logger::Builder;
use lazy_static::lazy_static;
use num_cpus;
use serde_json::json;
use std::time::SystemTime;

lazy_static! {
    static ref START_TIME: SystemTime = SystemTime::now();
}

#[get("/")]
async fn root() -> impl Responder {
    let uptime = match SystemTime::now().duration_since(*START_TIME) {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    };
    HttpResponse::Ok().json(json!({
        "uptime": uptime,
        "status": "server is healthy!"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let thread_count = num_cpus::get() - 6;

    Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    println!("{:?}: Server is running on port: {}", *START_TIME, 8080);

    HttpServer::new(|| App::new().service(root))
        .workers(thread_count)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

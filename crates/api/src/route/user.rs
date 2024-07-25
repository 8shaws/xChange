use actix_web::web;

async fn login() -> &'static str {
    "Login"
}

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").route("/login", web::post().to(login)));
}

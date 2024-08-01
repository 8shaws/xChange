use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::{middlewares::extract_client_id::ExtractClientId, types::AppState};

use common::types::order::NewOrder;

async fn create_new_order(
    app_state: web::Data<AppState>,
    order: web::Json<NewOrder>,
) -> impl Responder {
    HttpResponse::Ok().json(json!("Create order"))
}

pub fn order_config(cfg: &mut web::ServiceConfig, app_state: web::Data<AppState>) {
    cfg.service(
        web::scope("/orders")
            .wrap(ExtractClientId)
            .service(web::resource("/").route(web::post().to(create_new_order))),
    );
}

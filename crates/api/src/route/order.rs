use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use serde_json::json;

use crate::{
    middlewares::{
        extract_client_id::{ExtractClientId, IdKey},
        verify_user::VerifyUser,
    },
    redis::RedisClient,
    types::AppState,
};

use common::types::order::NewOrder;

async fn create_new_order(
    app_state: web::Data<AppState>,
    order: web::Json<NewOrder>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let id_key = req.extensions().get::<IdKey>().cloned();
    match id_key {
        Some(id) => {
            let order = order.into_inner();
            let pool = app_state.redis_pool.clone();
            let order_res = RedisClient::place_order_and_wait(&pool, order).await;
            match order_res {
                Ok(res) => Ok(HttpResponse::Ok().json(json!({
                    "status": "ok",
                    "message": "Order created successfully",
                    "order": res
                }))),
                Err(_) => Ok(HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Error creating order"
                }))),
            }
        }
        None => Ok(HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Invalid token"
        }))),
    }
}

pub fn order_config(cfg: &mut web::ServiceConfig, app_state: web::Data<AppState>) {
    let verify_user_middle = VerifyUser::new(app_state.clone());
    cfg.service(
        web::scope("/orders")
            .wrap(verify_user_middle)
            .wrap(ExtractClientId)
            .service(web::resource("/").route(web::post().to(create_new_order))),
    );
}

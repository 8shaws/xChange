use crate::db::user_db_fn::is_user_verified;
use crate::{auth::middleware::IdKey, models::AppState};
use actix_service::Service;
use actix_web::web::Data;
use actix_web::{
    body::EitherBody,
    dev::{ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use serde_json::json;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct VerifyUser {
    app_state: Data<AppState>,
}

impl VerifyUser {
    pub fn new(app_state: Data<AppState>) -> Self {
        Self { app_state }
    }
}

impl<S, B: 'static> Transform<S, ServiceRequest> for VerifyUser
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifyUserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(VerifyUserMiddleware {
            service: Rc::new(service),
            app_state: self.app_state.clone(),
        })
    }
}

pub struct VerifyUserMiddleware<S> {
    service: Rc<S>,
    app_state: Data<AppState>,
}

impl<S, B: 'static> Service<ServiceRequest> for VerifyUserMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);
        let id_key = req.extensions().get::<IdKey>().cloned();

        let db_pool = self.app_state.db_pool.clone();

        let fut = async move {
            let mut conn = db_pool.get().unwrap();

            match id_key {
                Some(id) => {
                    let verified = is_user_verified(&mut conn, id.0);
                    match verified {
                        Ok(verified) => {
                            if verified {
                                let res = srv.call(req).await?;
                                let mut res = res.map_into_left_body();
                                Ok(res)
                            } else {
                                let response = HttpResponse::Forbidden().json(json!({
                                    "message": "User Not Verified for this operation!",
                                    "status": "Error"
                                }));
                                let service_response =
                                    req.into_response(response.map_into_left_body());
                                Ok(service_response)
                            }
                        }
                        Err(_) => {
                            let response = HttpResponse::InternalServerError().json(json!({
                                "message": "User Verification Failed!",
                                "status": "Error"
                            }));
                            let service_response = req.into_response(response.map_into_left_body());
                            Ok(service_response)
                        }
                    }
                }
                None => {
                    let response = HttpResponse::Forbidden().json(json!({
                        "msg": "No authentication token found",
                        "status": "Error"
                    }));
                    let service_response = req.into_response(response.map_into_left_body());
                    Ok(service_response)
                }
            }
        };

        Box::pin(fut)
    }
}

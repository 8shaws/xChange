use super::utils::verify_token;
use actix_service::Service;
use actix_web::{
    body::EitherBody,
    cookie::{time::Duration, CookieBuilder},
    dev::{ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use serde_json::json;
use std::rc::Rc;
use std::task::{Context, Poll};

// middleware to extract the client ID from the request
pub struct ExtractClientId;

impl<S, B> Transform<S, ServiceRequest> for ExtractClientId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = ExtractClientIdMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ExtractClientIdMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct ExtractClientIdMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ExtractClientIdMiddleware<S>
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

        let mut jwt = String::new();

        // Extract JWT from the Authorization header
        if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    jwt = auth_str[7..].to_string();
                } else {
                    let response = HttpResponse::Forbidden().json(json!({
                        "msg": "Invalid authorization header, make sure to use the Bearer scheme",
                        "status": "Error"
                    }));
                    let service_response = req.into_response(response.map_into_right_body());
                    return Box::pin(async { Ok(service_response) });
                }
            }
        }

        // Extract JWT from cookies or query parameters if not in header
        if jwt.is_empty() {
            if let Some(cookie) = req.cookie("jwt") {
                jwt = cookie.value().to_string();
            } else if let Some(query_jwt) = req
                .query_string()
                .split('&')
                .find(|&q| q.starts_with("jwt="))
            {
                jwt = query_jwt[4..].to_string();
            }
        }

        let fut = async move {
            if !jwt.is_empty() {
                match verify_token(&jwt) {
                    Ok(id) => {
                        req.extensions_mut().insert(id.clone());
                        req.extensions_mut().insert(jwt.clone());

                        let res = srv.call(req).await?;

                        //set the cookie
                        let cookie = CookieBuilder::new("jwt", jwt.clone())
                            .http_only(true)
                            .path("/")
                            .secure(false)
                            .max_age(Duration::days(7))
                            .finish();

                        let mut res = res.map_into_left_body();
                        res.response_mut().headers_mut().append(
                            header::SET_COOKIE,
                            header::HeaderValue::from_str(&cookie.to_string()).unwrap(),
                        );

                        Ok(res)
                    }
                    Err(_) => {
                        let response = HttpResponse::Forbidden().json(json!({
                            "msg": "Auth error",
                            "status": "Error"
                        }));
                        let service_response = req.into_response(response.map_into_right_body());
                        Ok(service_response)
                    }
                }
            } else {
                let response = HttpResponse::Forbidden().json(json!({
                    "msg": "No authentication token found",
                    "status": "Error"
                }));
                let service_response = req.into_response(response.map_into_right_body());
                Ok(service_response)
            }
        };

        Box::pin(fut)
    }
}

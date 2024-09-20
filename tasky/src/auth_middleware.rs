use std::future::{Ready, ready};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use tonic::codegen::futures_core::future::LocalBoxFuture;

#[derive(Clone)]
pub struct UserData {
    pub user_id: i32
}

pub struct Auth;

impl Auth {

    pub fn new() -> Self {
        Auth {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let user_id = req.headers().get("X-CodeCanvas-UserId");
        if user_id.is_some() {
            let user_id_string = user_id.unwrap().to_str().unwrap();
            let id = user_id_string.parse::<i32>();
            if id.is_err() {
                return Box::pin(async {
                    Err(ErrorUnauthorized("No user id provided"))
                });
            }
            req.extensions_mut().insert(UserData{user_id: id.unwrap()});
        } else {
            return Box::pin(async {
                Err(ErrorUnauthorized("No user id provided"))
            });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
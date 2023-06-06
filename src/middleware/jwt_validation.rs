/*use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use std::future::{ready, Ready};

pub struct JwtValidation;
pub struct JwtValidationTransform<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for JwtValidation
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtValidationTransform<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtValidationTransform { service }))
    }
}

impl<S, B> Service<ServiceRequest> for JwtValidationTransform<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {}
}*/

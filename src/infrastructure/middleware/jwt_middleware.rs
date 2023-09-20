use std::future::{ready, Ready};

use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, HttpRequest,
};
use futures_util::future::LocalBoxFuture;
use log::error;
use crate::model::jwt_model::JwtToken;

#[derive(Clone)]
pub struct Auth;
pub struct AuthMiddleWare<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let bearer_token = request.headers().get("authorization");

        if let Some(bearer_token) = bearer_token {
            let token_string = match bearer_token.to_str() {
                Ok(token_str) => token_str,
                Err(err) => {
                    error!("{err}");
                    let (request, _pl) = request.into_parts();

                    let response = HttpResponse::BadRequest()
                        .body("bearer jwt decoding failed")
                        .map_into_right_body();
        
                    return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                }
            };

            let token_value: String = token_string.chars().skip(7).collect();
            let jwt_token = JwtToken { token: token_value };
            let jwt_claims = match JwtToken::jwt_validate_token(jwt_token) {
                Ok(jwt_token) => jwt_token,
                Err(err) => {
                    let (request, _pl) = request.into_parts();
                    let response = HttpResponse::Unauthorized()
                        .body(format!("{err}"))
                        .map_into_right_body();
                    return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                }
            };

            request.extensions_mut()
                .insert(AuthenticatedClaims {
                    user_id: jwt_claims.custom.user_id,
                    username: jwt_claims.custom.username,
                });

            let res = self.service.call(request);

            return Box::pin(async move {
                res.await.map(ServiceResponse::map_into_left_body)
            });
        }

        let (request, _pl) = request.into_parts();

        let response = HttpResponse::BadRequest()
            .body("no bearer jwt found")
            .map_into_right_body();

        Box::pin(async { Ok(ServiceResponse::new(request, response)) })
    }
}
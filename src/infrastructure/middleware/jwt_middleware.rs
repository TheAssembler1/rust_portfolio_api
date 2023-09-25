use std::future::{ready, Ready};

use crate::{
    infrastructure::api_error::{self, ApiError},
    model::jwt_model::{AuthenticatedClaims, JwtToken},
};
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use log::error;

#[derive(Clone)]
pub struct Auth;
pub struct AuthMiddleware<S> {
    service: S,
}

const BEARER_AUTH_HEADER_LOCATION: usize = 7;

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
                    let response = ApiError::JwtInternalError {
                        message: "jwt decoding failed".to_string(),
                    }
                    .error_response()
                    .map_into_right_body();

                    return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                }
            };
            let token_value: String = token_string.chars().skip(BEARER_AUTH_HEADER_LOCATION).collect();
            let jwt_token = JwtToken {
                access_token: token_value,
            };
            let jwt_claims = match JwtToken::jwt_validate_token(jwt_token) {
                Ok(jwt_token) => jwt_token,
                Err(err) => {
                    let (request, _pl) = request.into_parts();
                    let response = ApiError::Unauthorized {
                        message: err.to_string(),
                    }
                    .error_response()
                    .map_into_right_body();

                    return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
                }
            };

            request.extensions_mut().insert(AuthenticatedClaims {
                user_id: jwt_claims.custom.user_id,
            });

            let res = self.service.call(request);

            return Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) });
        }

        let (request, _pl) = request.into_parts();
        let response = ApiError::JwtNotFound.error_response().map_into_right_body();

        Box::pin(async { Ok(ServiceResponse::new(request, response)) })
    }
}

use actix_web::{
    http::StatusCode,
    HttpResponse, HttpResponseBuilder, error,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use log::error;

#[derive(Debug, Display, Error, Deserialize, Serialize)]
pub enum ApiError {
    #[display(fmt = "{}", message)]
    Unauthorized { message: String },
    DbError { message: String },
    DbPoolError,
    JwtNotFound,
    JwtInternalError { message: String },
}

impl error::ResponseError<> for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::Unauthorized { message } => {
                HttpResponseBuilder::new(StatusCode::UNAUTHORIZED).json(ApiError::Unauthorized {
                    message: message.to_owned(),
                })
            },
            ApiError::DbError { message } => {
                HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(ApiError::DbError {
                    message: message.to_owned(),
                })
            },
            ApiError::DbPoolError => {
                error!("failed to get connection from db pool");
                HttpResponse::InternalServerError().finish()
            },
            ApiError::JwtNotFound => {
                HttpResponse::NotFound().finish()
            },
            ApiError::JwtInternalError { message } => {
                HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR).json(ApiError::DbError {
                    message: message.to_owned(),
                })
            }
        }
    }
}

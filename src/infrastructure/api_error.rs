use actix_web::{HttpResponse, error, http::{header::ContentType, StatusCode}, HttpResponseBuilder};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum ApiError {
    #[display(fmt = "status: {}, message: {}", status, message)]
    Internal {
        message: String,
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ApiError::StdError { status, message } => {
                HttpResponseBuilder::status(status).body(body)
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        self.status_code()
    }
}
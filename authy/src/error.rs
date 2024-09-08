use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use hmac::digest::InvalidLength;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("BAD_REQUEST")]
    BadRequest,
    #[error("FORBIDDEN")]
    Forbidden,
    #[error("INTERNAL_SERVER_ERROR")]
    InternalServerError,
    #[error("UNAUTHORIZED")]
    Unauthorized
}

#[derive(Serialize)]
struct ResponseBody {
    message: String,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let response_body = ResponseBody {
            message: format!("{}", self)
        };
        HttpResponse::build(self.status_code()).json(response_body)
    }
}


impl<T> From<ApiError> for Result<T, ApiError> {
    fn from(error: ApiError) -> Self {
        Err(error)
    }
}

impl From<InvalidLength> for ApiError {
    fn from(value: InvalidLength) -> Self {
        ApiError::InternalServerError
    }
}

impl From<jwt::error::Error> for ApiError {
    fn from(value: jwt::error::Error) -> Self {
        ApiError::InternalServerError
    }
}
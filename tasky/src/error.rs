use std::num::TryFromIntError;

use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

/// The api error
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("BAD_REQUEST")]
    BadRequest,
    #[error("FORBIDDEN")]
    Forbidden,
    #[error("INTERNAL_SERVER_ERROR")]
    InternalServerError,
    #[error("UNAUTHORIZED")]
    Unauthorized,
}

/// The response body
#[derive(Serialize)]
struct ResponseBody {
    message: String,
}

/// Implements response error trait for api error
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Forbidden => StatusCode::FORBIDDEN,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let response_body = ResponseBody {
            message: format!("{}", self),
        };
        HttpResponse::build(self.status_code()).json(response_body)
    }
}

impl From<tonic::Status> for ApiError {
    fn from(value: tonic::Status) -> Self {
        ApiError::InternalServerError
    }
}

impl From<TryFromIntError> for ApiError {
    fn from(value: TryFromIntError) -> Self {
        ApiError::BadRequest
    }
}

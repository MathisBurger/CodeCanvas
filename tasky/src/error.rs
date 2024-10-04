use std::num::TryFromIntError;

use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

/// The api error
#[derive(Debug, Error, Serialize)]
pub enum ApiError {
    #[error("BAD_REQUEST")]
    BadRequest { message: String },
    #[error("FORBIDDEN")]
    Forbidden { message: String },
    #[error("INTERNAL_SERVER_ERROR")]
    InternalServerError { message: String },
    #[error("UNAUTHORIZED")]
    Unauthorized { message: String },
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
            ApiError::BadRequest { message: _ } => StatusCode::BAD_REQUEST,
            ApiError::Forbidden { message: _ } => StatusCode::FORBIDDEN,
            ApiError::InternalServerError { message: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized { message: _ } => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let response_body = ResponseBody {
            message: format!(
                "{}",
                match self {
                    ApiError::BadRequest { message } => message,
                    ApiError::Forbidden { message } => message,
                    ApiError::InternalServerError { message } => message,
                    ApiError::Unauthorized { message } => message,
                }
            ),
        };
        HttpResponse::build(self.status_code()).json(response_body)
    }
}

impl From<tonic::Status> for ApiError {
    fn from(_: tonic::Status) -> Self {
        ApiError::InternalServerError {
            message: "Invalid grpc call from sub microservice".to_string(),
        }
    }
}

impl From<TryFromIntError> for ApiError {
    fn from(_: TryFromIntError) -> Self {
        ApiError::BadRequest {
            message: "Cannot convert between integer bit sizes".to_string(),
        }
    }
}

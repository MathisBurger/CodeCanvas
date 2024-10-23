use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use hmac::digest::InvalidLength;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
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

#[derive(Serialize)]
struct ResponseBody {
    message: String,
}

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
            message: (match self {
                ApiError::BadRequest { message } => message,
                ApiError::Forbidden { message } => message,
                ApiError::InternalServerError { message } => message,
                ApiError::Unauthorized { message } => message,
            })
            .to_string(),
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
    fn from(_value: InvalidLength) -> Self {
        ApiError::InternalServerError {
            message: "Invalid text length".to_string(),
        }
    }
}

impl From<jwt::error::Error> for ApiError {
    fn from(_value: jwt::error::Error) -> Self {
        ApiError::InternalServerError {
            message: "Error with JWT".to_string(),
        }
    }
}

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ErrorResponse {
    NotFound(String),
    BadRequest(String),
    Conflict(String),
    InternalServerError,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponseSerializable {
    pub status_code: u16,
    pub status: String,
    pub error: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        let status_code = match self {
            ErrorResponse::NotFound(_) => StatusCode::NOT_FOUND,
            ErrorResponse::BadRequest(_) => StatusCode::BAD_REQUEST,
            ErrorResponse::Conflict(_) => StatusCode::CONFLICT,
            ErrorResponse::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let status = status_code.canonical_reason().unwrap_or("").to_string();

        let error = match self {
            ErrorResponse::NotFound(e) => e.to_string(),
            ErrorResponse::BadRequest(e) => e.to_string(),
            ErrorResponse::Conflict(e) => e.to_string(),
            ErrorResponse::InternalServerError => "Internal Server Error".to_string(),
        };

        HttpResponse::build(status_code).json(ErrorResponseSerializable {
            status_code: status_code.as_u16(),
            status,
            error: error.to_string(),
        })
    }
}

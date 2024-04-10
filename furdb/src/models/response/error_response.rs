use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

use super::api_response::ApiResponseSerializable;
use std::fmt;

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum ErrorResponse {
    NotFound(String),
    BadRequest(String),
    Conflict(String),
    InternalServerError,
}
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorResponse::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ErrorResponse::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ErrorResponse::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ErrorResponse::InternalServerError => write!(f, "Internal Server Error"),
        }
    }
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        let (response, status_code) = ApiResponseSerializable::generate_error(self);
        HttpResponse::build(status_code).json(response)
    }
}

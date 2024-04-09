use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub struct ErrorResponse {
    pub status_code: StatusCode,
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponseSerializable {
    pub status_code: u16,
    pub status: String,
    pub error: String,
}

impl Into<ErrorResponseSerializable> for ErrorResponse {
    fn into(self) -> ErrorResponseSerializable {
        ErrorResponseSerializable {
            status_code: self.status_code.as_u16(),
            status: self
                .status_code
                .canonical_reason()
                .unwrap_or("")
                .to_string(),
            error: self.error,
        }
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code)
            .json(Into::<ErrorResponseSerializable>::into(self.to_owned()))
    }
}

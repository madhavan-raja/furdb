use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub struct ErrorResponse {
    pub status_code: u16,
    pub error: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse {
        let status_code = StatusCode::from_u16(self.status_code).unwrap_or_default();
        HttpResponse::build(status_code).json(self)
    }
}
use std::error::Error;

use chrono::Local;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum ResponseType<T: serde::Serialize> {
    Success(T),
    Error(ErrorResponse),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct ApiResponse<T: serde::Serialize> {
    timestamp: String,
    status: u16,
    response: ResponseType<T>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub(crate) fn new(error: String) -> Self {
        Self { error }
    }
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn new(response: Result<T, Box<dyn Error>>) -> Self {
        let response = match response {
            Ok(response) => ResponseType::Success(response),
            Err(error) => ResponseType::Error(ErrorResponse::new(error.to_string())),
        };

        ApiResponse {
            timestamp: Local::now().to_string(),
            status: 0,
            response,
        }
    }
}

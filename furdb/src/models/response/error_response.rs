use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum ErrorResponse {
    NotFound(String),
    BadRequest(String),
    Conflict(String),
    InternalServerError,
}

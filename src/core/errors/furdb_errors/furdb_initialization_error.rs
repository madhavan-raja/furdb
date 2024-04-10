use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Error initializing FurDB";

#[derive(Error, Debug)]
pub enum FurDBInitializationError {
    #[error("{BASE_ERROR_MESSAGE}: Permission denied")]
    PermissionDenied,
    #[error("{BASE_ERROR_MESSAGE}: Invalid path")]
    InvalidPath,
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

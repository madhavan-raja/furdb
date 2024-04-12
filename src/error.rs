use thiserror::Error;

use crate::core::error::FurDBInitializationError;

const APPLICATION_ERROR_MESSAGE: &str = "Cannot start FurDB";

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("{APPLICATION_ERROR_MESSAGE}: Error initializing FurDB")]
    Initialization(#[from] FurDBInitializationError),
    #[error("{APPLICATION_ERROR_MESSAGE}: Error starting server")]
    ServerStart,
    #[error("{APPLICATION_ERROR_MESSAGE}")]
    Other(String),
}

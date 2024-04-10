use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Error reading Database";

#[derive(Error, Debug)]
pub enum DatabaseReadError {
    #[error("{BASE_ERROR_MESSAGE}: Database not found")]
    NotFound,
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

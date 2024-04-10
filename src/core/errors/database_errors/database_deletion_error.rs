use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Error deleting Database";

#[derive(Error, Debug)]
pub enum DatabaseDeletionError {
    #[error("{BASE_ERROR_MESSAGE}: Database not found")]
    NotFound,
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

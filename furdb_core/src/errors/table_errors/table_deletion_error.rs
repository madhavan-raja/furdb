use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Error deleting Table";

#[derive(Error, Debug)]
pub enum TableDeletionError {
    #[error("{BASE_ERROR_MESSAGE}: Table not found")]
    NotFound,
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

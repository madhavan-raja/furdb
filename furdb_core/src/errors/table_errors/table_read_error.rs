use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Error reading Table";

#[derive(Error, Debug)]
pub enum TableReadError {
    #[error("{BASE_ERROR_MESSAGE}: Table not found")]
    NotFound,
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

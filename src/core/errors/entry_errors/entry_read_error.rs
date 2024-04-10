use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Error reading entry";

#[derive(Error, Debug)]
pub enum EntryReadError {
    #[error("{BASE_ERROR_MESSAGE}: Invalid index")]
    InvalidIndex,
    #[error("{BASE_ERROR_MESSAGE}: Invalid Column")]
    InvalidColumn,
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

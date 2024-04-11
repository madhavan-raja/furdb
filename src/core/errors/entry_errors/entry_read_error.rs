use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Cannot read Entry";

#[derive(Error, Debug)]
pub enum EntryReadError {
    #[error("{BASE_ERROR_MESSAGE}: Invalid index")]
    InvalidIndex,
    #[error("{BASE_ERROR_MESSAGE}: Invalid Column")]
    InvalidColumn,
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Cannot create Table";

#[derive(Error, Debug)]
pub enum TableCreationError {
    #[error("{BASE_ERROR_MESSAGE}: Table already exists")]
    AlreadyExists,
    #[error("{BASE_ERROR_MESSAGE}: Invalid Table ID")]
    InvalidId,
    #[error("{BASE_ERROR_MESSAGE}: Size of row must be a multiple of 8")]
    ColumnsUnfit,
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

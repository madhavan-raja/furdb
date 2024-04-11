use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Cannot insert Entry";

#[derive(Error, Debug)]
pub enum EntryInsertionError {
    #[error("{BASE_ERROR_MESSAGE}: Number of columns does not match number of values")]
    ColumnMismatch,
    #[error("{BASE_ERROR_MESSAGE}: Size of value exceeds column size")]
    ColumnOverflow,
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

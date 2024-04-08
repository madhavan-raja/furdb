use thiserror::Error;

#[derive(Error, Debug)]
pub enum InsertionQueryError {
    #[error("Insertion Query Error: Number of columns does not match number of values")]
    ColumnMismatch,
    #[error("Insertion Query Error: Size of value exceeds column size")]
    ColumnOverflow,
    #[error("Insertion Query Error")]
    OtherError(String),
}
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadQueryError {
    #[error("Read Query Error: Invalid index")]
    InvalidIndex,
    #[error("Read Query Error: Invalid Column")]
    InvalidColumn,
    #[error("Read Query Error")]
    OtherError(String),
}
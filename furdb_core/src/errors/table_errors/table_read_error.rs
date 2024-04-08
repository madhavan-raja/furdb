use thiserror::Error;

#[derive(Error, Debug)]
pub enum TableReadError {
    #[error("Error reading Table: Table not found")]
    NotFound,
    #[error("Error reading Table")]
    OtherError(String),
}
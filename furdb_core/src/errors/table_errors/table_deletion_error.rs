use thiserror::Error;

#[derive(Error, Debug)]
pub enum TableDeletionError {
    #[error("Error deleting Table: Table not found")]
    NotFound,
    #[error("Error deleting Table")]
    OtherError(String),
}
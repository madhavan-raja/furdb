use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseDeletionError {
    #[error("Error deleting Database: Database not found")]
    NotFound,
    #[error("Error deleting Database")]
    OtherError(String),
}
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseReadError {
    #[error("Error reading Database: Database not found")]
    NotFound,
    #[error("Error reading Database")]
    OtherError(String),
}
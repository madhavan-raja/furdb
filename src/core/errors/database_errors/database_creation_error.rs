use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Cannot create Database";

#[derive(Error, Debug)]
pub enum DatabaseCreationError {
    #[error("{BASE_ERROR_MESSAGE}: Database already exists")]
    AlreadyExists,
    #[error("{BASE_ERROR_MESSAGE}: Invalid Database ID")]
    InvalidId,
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

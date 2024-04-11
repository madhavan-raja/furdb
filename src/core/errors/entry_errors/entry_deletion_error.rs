use thiserror::Error;

const BASE_ERROR_MESSAGE: &str = "Cannot delete Entry";

#[derive(Error, Debug)]
pub enum EntryDeletionError {
    #[error("{BASE_ERROR_MESSAGE}")]
    OtherError(String),
}

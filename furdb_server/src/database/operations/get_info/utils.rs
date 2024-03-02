use furdb_core::{Database, DatabaseInfo};
use std::error::Error;
use std::path::PathBuf;

use crate::config::WORKSPACE_PATH;

pub(crate) fn get_database(database_id: &str) -> Result<Database, Box<dyn Error>> {
    let working_dir = PathBuf::from(WORKSPACE_PATH);
    let mut db_path = working_dir.clone();
    db_path.push(database_id);

    Database::get_database(db_path)
}

pub(crate) fn get_database_path(database_id: &str) -> PathBuf {
    let working_dir = PathBuf::from(WORKSPACE_PATH);
    let mut db_path = working_dir.clone();
    db_path.push(database_id);

    db_path
}

use furdb_core::{Database, DatabaseInfo};
use std::error::Error;
use std::path::PathBuf;

use crate::config::WORKSPACE_PATH;

pub(crate) fn create_database(
    database_id: &str,
    database_name: &str,
) -> Result<Database, Box<dyn Error>> {
    let working_dir = PathBuf::from(WORKSPACE_PATH);
    let mut db_path = working_dir.clone();
    db_path.push(database_id);

    let db_info = DatabaseInfo::new(&database_name)?;

    Database::create_database(db_path.clone(), db_info)?;

    Database::get_database(db_path)
}

pub(crate) fn get_database(database_id: &str) -> Result<Database, Box<dyn Error>> {
    let working_dir = PathBuf::from(WORKSPACE_PATH);
    let mut db_path = working_dir.clone();
    db_path.push(database_id);

    Database::get_database(db_path)
}

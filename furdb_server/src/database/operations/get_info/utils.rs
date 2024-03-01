use furdb_core::{Database, DatabaseInfo};
use std::error::Error;
use std::path::PathBuf;

use crate::config::WORKSPACE_PATH;

pub(crate) fn get_db(db_id: &str, db_name: Option<String>) -> Result<Database, Box<dyn Error>> {
    let working_dir = PathBuf::from(WORKSPACE_PATH);

    let mut db_path = working_dir.clone();
    db_path.push(db_id);

    let db_info = if db_name.is_some() {
        Some(DatabaseInfo::new(&db_name.as_ref().unwrap())?)
    } else {
        None
    };

    Database::new(db_path, db_info)
}

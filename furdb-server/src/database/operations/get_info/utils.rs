use furdb_core::{FurDB, FurDBInfo};
use std::error::Error;
use std::path::PathBuf;

use crate::config::WORKSPACE_PATH;

pub(crate) fn get_db(
    working_dir: Option<PathBuf>,
    db_id: &str,
    db_name: Option<String>,
) -> Result<FurDB, Box<dyn Error>> {
    let working_dir = if working_dir.is_some() {
        working_dir.unwrap()
    } else {
        PathBuf::from(WORKSPACE_PATH)
    };

    let mut db_path = working_dir.clone();
    db_path.push(db_id);

    let db_info = if db_name.is_some() {
        Some(FurDBInfo::new(&db_name.as_ref().unwrap())?)
    } else {
        None
    };

    FurDB::new(db_path, db_info)
}

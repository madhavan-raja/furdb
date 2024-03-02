use std::path::PathBuf;

use crate::config::get_fur_directory;

pub(crate) fn get_database_path(database_id: &str) -> PathBuf {
    let mut db_path = get_fur_directory().clone();
    db_path.push(database_id);

    println!("db_path: {:?}", db_path);

    db_path
}

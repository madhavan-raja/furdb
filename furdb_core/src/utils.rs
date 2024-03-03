use std::{error::Error, path::PathBuf};

use crate::Database;

pub(crate) fn get_fur_path() -> Result<PathBuf, Box<dyn Error>> {
    let fur_path = PathBuf::from(std::env::var("FUR_DIRECTORY")?);
    Ok(fur_path)
}

pub(crate) fn get_database_path(database_id: &str) -> Result<PathBuf, Box<dyn Error>> {
    let database_path = get_fur_path()?.join(database_id);
    Ok(database_path)
}

pub(crate) fn get_database_config_path(database_id: &str) -> Result<PathBuf, Box<dyn Error>> {
    let database_config_path = get_database_path(database_id)?.join("database_config.json");
    Ok(database_config_path)
}

pub(crate) fn get_all_tables_path(database_id: &str) -> Result<PathBuf, Box<dyn Error>> {
    let database_path = get_database_path(database_id)?.join("tables");
    Ok(database_path)
}

pub(crate) fn get_table_path(
    database: &Database,
    table_id: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let table_path = get_all_tables_path(&database.get_database_id())?.join(table_id);
    Ok(table_path)
}

pub(crate) fn get_table_config_path(
    database: &Database,
    table_id: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let table_path = get_table_path(&database, table_id)?.join("table_config.json");
    Ok(table_path)
}

pub(crate) fn get_table_data_path(
    database: &Database,
    table_id: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let table_path = get_table_path(&database, table_id)?.join("data");
    Ok(table_path)
}

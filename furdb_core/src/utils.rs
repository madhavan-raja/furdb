use std::{error::Error, path::PathBuf};

pub(crate) fn get_database_path(
    fur_directory: &PathBuf,
    database_id: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let database_path = fur_directory.join(database_id);
    Ok(database_path)
}

pub(crate) fn get_database_config_path(
    fur_directory: &PathBuf,
    database_id: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let database_config_path =
        get_database_path(&fur_directory, database_id)?.join("database_config.json");
    Ok(database_config_path)
}

pub(crate) fn get_all_tables_path(
    fur_directory: &PathBuf,
    database_id: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let database_path = get_database_path(&fur_directory, database_id)?.join("tables");
    Ok(database_path)
}

pub(crate) fn get_table_path(
    fur_directory: &PathBuf,
    database_id: &str,
    table_id: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let table_path = get_all_tables_path(&fur_directory, &database_id)?.join(table_id);
    Ok(table_path)
}

pub(crate) fn get_table_config_path(
    fur_directory: &PathBuf,
    database_id: &str,
    table_id: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let table_path =
        get_table_path(&fur_directory, &database_id, table_id)?.join("table_config.json");
    Ok(table_path)
}

pub(crate) fn get_table_data_path(
    fur_directory: &PathBuf,
    database_id: &str,
    table_id: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let table_path = get_table_path(&fur_directory, &database_id, table_id)?.join("data");
    Ok(table_path)
}

use std::path::PathBuf;

pub fn get_database_path(fur_directory: &PathBuf, database_id: &str) -> PathBuf {
    let database_path = fur_directory.join(database_id);
    database_path
}

pub fn get_database_config_path(fur_directory: &PathBuf, database_id: &str) -> PathBuf {
    let database_config_path =
        get_database_path(&fur_directory, database_id).join("database_config.json");
    database_config_path
}

pub fn get_all_tables_path(fur_directory: &PathBuf, database_id: &str) -> PathBuf {
    let database_path = get_database_path(&fur_directory, database_id).join("tables");
    database_path
}

pub fn get_table_path(fur_directory: &PathBuf, database_id: &str, table_id: &str) -> PathBuf {
    let table_path = get_all_tables_path(&fur_directory, &database_id).join(table_id);
    table_path
}

pub fn get_table_config_path(
    fur_directory: &PathBuf,
    database_id: &str,
    table_id: &str,
) -> PathBuf {
    let table_path =
        get_table_path(&fur_directory, &database_id, table_id).join("table_config.json");
    table_path
}

pub fn get_table_data_path(fur_directory: &PathBuf, database_id: &str, table_id: &str) -> PathBuf {
    let table_path = get_table_path(&fur_directory, &database_id, table_id).join("data");
    table_path
}

pub fn get_sortfile_path(fur_directory: &PathBuf, database_id: &str, table_id: &str) -> PathBuf {
    let sortfile_path = get_table_path(&fur_directory, &database_id, table_id).join("sortfile");
    sortfile_path
}

pub fn is_id_valid(id: &str) -> bool {
    id.chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

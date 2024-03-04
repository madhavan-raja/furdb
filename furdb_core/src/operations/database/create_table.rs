use crate::models;
use crate::utils;
use std::error::Error;

impl models::database::Database {
    pub fn create_table(
        &self,
        table_id: &str,
        table_name: Option<&str>,
        table_columns: Vec<models::column::Column>,
    ) -> Result<models::table::Table, Box<dyn Error>> {
        let config = self.get_config();
        let database_info = self.get_database_info();

        let table_info = models::table::TableInfo::new(
            &database_info.get_database_id(),
            &table_id,
            &table_name.unwrap_or(&table_id),
            &table_columns,
        )?;

        let table = models::table::Table::new(&config, &table_info)?;

        let table_path = utils::get_table_path(
            &config.fur_directory,
            &database_info.get_database_id(),
            &table_id,
        )?;
        let table_config_path = utils::get_table_config_path(
            &config.fur_directory,
            &database_info.get_database_id(),
            &table_id,
        )?;
        let table_data_path = utils::get_table_data_path(
            &config.fur_directory,
            &database_info.get_database_id(),
            &table_id,
        )?;

        std::fs::create_dir(&table_path)?;
        std::fs::write(&table_config_path, serde_json::to_string(&table_info)?)?;
        std::fs::write(table_data_path, "")?;

        Ok(table)
    }
}

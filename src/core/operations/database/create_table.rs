use crate::core::utils;

use crate::core::models::column::Column;
use crate::core::models::database::Database;
use crate::core::models::table::Table;
use crate::core::models::table::TableInfo;

use crate::core::error::TableCreationError;
use std::io::ErrorKind;

impl Database {
    pub fn create_table(
        &self,
        table_id: &str,
        table_name: Option<&str>,
        table_columns: Vec<Column>,
    ) -> Result<Table, TableCreationError> {
        let config = self.get_config();
        let database_info = self.get_database_info();

        let table_info = TableInfo::new(
            &database_info.get_database_id(),
            table_id,
            table_name.unwrap_or(table_id),
            &table_columns,
        );

        if !utils::is_id_valid(table_id) {
            return Err(TableCreationError::InvalidId);
        }

        let table = Table::new(&config, &table_info);

        let table_path =
            utils::get_table_path(&config.workdir, &database_info.get_database_id(), table_id);
        let table_config_path = utils::get_table_config_path(
            &config.workdir,
            &database_info.get_database_id(),
            table_id,
        );
        let table_data_path =
            utils::get_table_data_path(&config.workdir, &database_info.get_database_id(), table_id);
        let table_sortfile_path =
            utils::get_sortfile_path(&config.workdir, &database_info.get_database_id(), table_id);

        if table_columns.iter().fold(0, |acc, x| acc + x.get_size()) % 8 != 0 {
            return Err(TableCreationError::ColumnsUnfit);
        }

        std::fs::create_dir(table_path).map_err(|e| match e.kind() {
            ErrorKind::AlreadyExists => TableCreationError::AlreadyExists,
            _ => TableCreationError::OtherError(e.to_string()),
        })?;

        let table_info_serialized = serde_json::to_string(&table_info)
            .map_err(|e| TableCreationError::OtherError(e.to_string()))?;

        std::fs::write(table_config_path, table_info_serialized)
            .map_err(|e| TableCreationError::OtherError(e.to_string()))?;
        std::fs::write(table_data_path, "")
            .map_err(|e| TableCreationError::OtherError(e.to_string()))?;
        std::fs::write(table_sortfile_path, "")
            .map_err(|e| TableCreationError::OtherError(e.to_string()))?;

        Ok(table)
    }
}

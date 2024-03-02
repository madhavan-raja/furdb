use furdb_core::DatabaseInfo;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetDatabaseResponse {
    pub database_info: DatabaseInfo,
    pub database_tables: Vec<String>,
}

impl GetDatabaseResponse {
    pub fn new(database_info: DatabaseInfo, database_tables: Vec<String>) -> Self {
        Self {
            database_info,
            database_tables,
        }
    }
}

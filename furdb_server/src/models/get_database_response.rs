use furdb_core::DatabaseInfo;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetDatabaseResponse {
    pub(crate) database_info: DatabaseInfo,
    pub(crate) database_tables: Vec<String>,
}

impl GetDatabaseResponse {
    pub(crate) fn new(database_info: DatabaseInfo, database_tables: Vec<String>) -> Self {
        Self {
            database_info,
            database_tables,
        }
    }
}

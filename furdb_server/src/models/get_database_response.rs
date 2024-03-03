use furdb_core::Table;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetDatabaseResponse {
    pub(crate) database_id: String,
    pub(crate) database_name: String,
    pub(crate) database_tables: Vec<Table>,
}

impl GetDatabaseResponse {
    pub(crate) fn new(database_id: &str, database_name: &str, database_tables: Vec<Table>) -> Self {
        Self {
            database_id: String::from(database_id),
            database_name: String::from(database_name),
            database_tables,
        }
    }
}

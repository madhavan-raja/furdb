use furdb_core::DatabaseInfo;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct DatabaseResponse {
    pub db_info: DatabaseInfo,
    pub tables: Vec<String>,
}

impl DatabaseResponse {
    pub fn new(db_info: DatabaseInfo, tables: Vec<String>) -> Self {
        DatabaseResponse { db_info, tables }
    }
}

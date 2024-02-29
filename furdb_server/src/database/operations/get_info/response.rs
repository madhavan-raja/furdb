use furdb_core::FurDBInfo;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct DatabaseResponse {
    pub db_info: FurDBInfo,
    pub tables: Vec<String>,
}

impl DatabaseResponse {
    pub fn new(db_info: FurDBInfo, tables: Vec<String>) -> Self {
        DatabaseResponse { db_info, tables }
    }
}

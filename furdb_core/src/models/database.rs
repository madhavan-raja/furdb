#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Database {
    pub(crate) database_id: String,
    pub(crate) database_name: String,
}

impl Database {
    pub fn get_database_id(&self) -> String {
        self.database_id.clone()
    }

    pub fn get_database_name(&self) -> String {
        self.database_name.clone()
    }
}

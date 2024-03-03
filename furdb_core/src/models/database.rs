use std::error::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Database {
    database_id: String,
    database_name: String,
}

impl Database {
    pub fn new(database_id: &str, database_name: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            database_id: String::from(database_id),
            database_name: String::from(database_name),
        })
    }

    pub fn get_database_id(&self) -> String {
        self.database_id.clone()
    }

    pub fn get_database_name(&self) -> String {
        self.database_name.clone()
    }
}

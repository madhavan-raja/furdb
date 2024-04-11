use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDatabaseParams {
    database_name: Option<String>,
}

impl CreateDatabaseParams {
    pub fn get_database_name(&self) -> Option<String> {
        self.database_name.to_owned()
    }
}

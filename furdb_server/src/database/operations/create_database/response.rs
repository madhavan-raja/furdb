#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct CreateDatabaseResponse {}

impl CreateDatabaseResponse {
    pub fn new() -> Self {
        Self {}
    }
}

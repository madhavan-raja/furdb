#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateDatabaseResponse {}

impl CreateDatabaseResponse {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

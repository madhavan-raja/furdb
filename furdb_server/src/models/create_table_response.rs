#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateTableResponse {}

impl CreateTableResponse {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

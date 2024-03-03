#[derive(serde::Serialize, serde::Deserialize)]
pub struct BlankSuccessResponse {}

impl BlankSuccessResponse {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

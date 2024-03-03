#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct BlankSuccessResponse {}

impl BlankSuccessResponse {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

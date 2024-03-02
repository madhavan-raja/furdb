#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct ApiResponse<T: serde::Serialize> {
    timestamp: u64,
    status: u16,
    response: T,
}

impl ApiResponse {
    pub fn new(response: T) -> Self {
        ApiResponse {
            timestamp: 0,
            status: 200,
            response,
        }
    }
}

use chrono::Local;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct ApiResponse<T: serde::Serialize> {
    timestamp: String,
    status: u16,
    response: T,
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn new(response: T) -> Self {
        ApiResponse {
            timestamp: Local::now().to_string(),
            status: 0,
            response,
        }
    }
}

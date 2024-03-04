#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetRowsResponse {
    data: Vec<Vec<u128>>,
}

impl GetRowsResponse {
    pub(crate) fn new(data: &[Vec<u128>]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            data: data.to_vec(),
        })
    }
}

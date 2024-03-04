#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct GetRowResponse {
    data: Vec<u128>,
}

impl GetRowResponse {
    pub(crate) fn new(data: &[u128]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            data: data.to_vec(),
        })
    }
}

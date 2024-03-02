use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct TableContentResponse {
    pub result: Vec<HashMap<String, String>>,
}

impl TableContentResponse {
    pub fn new(data: Vec<HashMap<String, String>>) -> Self {
        Self { result: data }
    }
}

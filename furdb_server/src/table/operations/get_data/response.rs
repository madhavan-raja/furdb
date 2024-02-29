use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct TableResponse {
    pub result: Vec<HashMap<String, String>>,
}

impl TableResponse {
    pub fn new(data: Vec<HashMap<String, String>>) -> Self {
        TableResponse { result: data }
    }
}

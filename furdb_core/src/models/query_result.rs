#[derive(serde::Serialize, serde::Deserialize)]
pub struct QueryResult {
    result_count: usize,
    results: Vec<Row>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Row {
    index: usize,
    data: Vec<u128>,
}

impl QueryResult {
    pub(crate) fn new(data: &[Row]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            result_count: data.len(),
            results: data.to_vec(),
        })
    }

    pub fn get_result_count(&self) -> usize {
        self.result_count
    }

    pub fn get_results(&self) -> Vec<Row> {
        self.results.to_owned()
    }
}

impl Row {
    pub(crate) fn new(index: usize, data: Vec<u128>) -> Self {
        Self { index, data }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_data(&self) -> Vec<u128> {
        self.data.to_owned()
    }
}

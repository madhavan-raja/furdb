#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetRowsResult {
    result_count: usize,
    results: Vec<Row>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Row {
    index: usize,
    data: Vec<u128>,
}

impl GetRowsResult {
    pub(crate) fn new(data: &[Vec<u128>]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            result_count: data.len(),
            results: data
                .iter()
                .enumerate()
                .map(|(index, row)| Row::new(index, row.to_vec()))
                .collect(),
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

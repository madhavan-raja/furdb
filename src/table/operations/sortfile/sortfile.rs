use std::error::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Sortfile {
    column_id: String,
    sortlist: Vec<u64>,
}

impl Sortfile {
    pub fn new(column_id: &str, sortlist: &[u64]) -> Result<Self, Box<dyn Error>> {
        let column_id = String::from(column_id);
        let sortlist = Vec::from(sortlist);
        Ok(Sortfile {
            column_id,
            sortlist,
        })
    }

    pub fn get_column_id(&self) -> String {
        self.column_id.clone()
    }
}

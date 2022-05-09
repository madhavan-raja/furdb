use std::error::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Sortfile {
    id: String,
    sortlist: Vec<u64>,
}

impl Sortfile {
    pub fn new(id: &str, sortlist: &[u64]) -> Result<Self, Box<dyn Error>> {
        let id = String::from(id);
        let sortlist = Vec::from(sortlist);
        Ok(Sortfile { id, sortlist })
    }

    pub fn get_column_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_sortlist(&self) -> Vec<u64> {
        self.sortlist.clone()
    }
}

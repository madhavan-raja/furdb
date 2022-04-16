use std::error::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Sortfile {
    column_name: String,
    sortlist: Vec<u64>,
}

impl Sortfile {
    pub fn new(column_name: &str, sortlist: &[u64]) -> Result<Self, Box<dyn Error>> {
        let column_name = String::from(column_name);
        let sortlist = Vec::from(sortlist);
        Ok(Sortfile {
            column_name,
            sortlist,
        })
    }

    pub fn dump(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

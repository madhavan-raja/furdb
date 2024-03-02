use std::error::Error;

use crate::Sortfile;

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

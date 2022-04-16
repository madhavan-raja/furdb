use crate::FurTable;
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

impl FurTable {
    pub fn dump(&self, sortfile_contents: &Sortfile) -> Result<(), Box<dyn Error>> {
        let column_id = sortfile_contents.get_column_id();
        let sortfile_contents = serde_json::to_string(sortfile_contents)?;
        let sortfile_path = Self::get_sortfile_path(&self.dir, &column_id);

        // std::fs::write(sortfile_path, sortfile_contents)?;

        println!("Writing: {:?} {:?}", sortfile_path, sortfile_contents);

        Ok(())
    }
}

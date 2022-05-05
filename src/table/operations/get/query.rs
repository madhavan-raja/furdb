use crate::{FurColumn, FurTable};
use std::collections::HashMap;
use std::error::Error;

impl FurTable {
    pub fn query(
        &mut self,
        column_id: &str,
        value: &str,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let sortfile = self.read_sortfile(column_id)?;

        todo!()
    }
}

use crate::{FurColumn, FurTable};
use std::collections::HashMap;
use std::error::Error;

impl FurTable {
    pub fn query(
        &mut self,
        column: FurColumn,
        value: &str,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        todo!()
    }
}

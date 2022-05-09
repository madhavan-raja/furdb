use crate::{FurColumn, FurTable};
use std::error::Error;

impl FurTable {
    pub fn query(
        &mut self,
        column: &FurColumn,
        value: &str,
    ) -> Result<Option<u64>, Box<dyn Error>> {
        let sortlist = self.read_sortfile(&column.get_id())?.get_sortlist();

        println!("{:?}", sortlist);

        let data_type = column.get_data_type();
        let converter_server = self.table_info.get_converter_server();

        let target_value = data_type.encode(value, column.get_size(), converter_server.clone())?;

        let mut left = 0 as u64;
        let mut right = sortlist.len() as u64 - 1;

        let column_id = column.get_id();

        while left <= right {
            let mid = (left + right) / 2;
            let mid_row_bin = self.get_row_bin(mid)?;
            let mid_value = mid_row_bin.get(&column_id).unwrap().clone();

            match data_type.compare(&mid_value, &target_value, converter_server.clone())? {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid - 1,
                std::cmp::Ordering::Equal => return Ok(Some(mid)),
            }
        }

        Ok(None)
    }
}

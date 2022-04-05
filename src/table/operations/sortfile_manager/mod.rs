use crate::{FurColumn, FurTable};
use std::error::Error;

impl FurTable {
    pub fn generate_sortfile_content(
        &mut self,
        column: &FurColumn,
    ) -> Result<Vec<u64>, Box<dyn Error>> {
        let rows = &self.get_all()?;
        let mut sortfile_content: Vec<u64> = (0..(rows.len() as u64)).collect();

        sortfile_content.sort_by(|a, b| {
            let bits_a = self
                .get_row_bin(*a)
                .unwrap()
                .get(&column.get_id())
                .unwrap()
                .clone();

            let bits_b = self
                .get_row_bin(*b)
                .unwrap()
                .get(&column.get_id())
                .unwrap()
                .clone();

            let data_type = column.get_data_type();
            let cmp = data_type
                .compare(&bits_a, &bits_b, self.table_info.get_converter_server())
                .unwrap();

            cmp
        });

        Ok(sortfile_content)
    }

    pub fn generate_sortfile_contents(
        &mut self,
        columns: &[FurColumn],
    ) -> Result<(), Box<dyn Error>> {
        for column in columns {
            let sortfile_content = self.generate_sortfile_content(column)?;
            println!("Sortfile: {}: {:?}", column.get_id(), sortfile_content);
        }

        Ok(())
    }

    pub fn generate_all_sortfiles(&mut self) -> Result<(), Box<dyn Error>> {
        let columns = self.table_info.get_columns().clone();

        self.generate_sortfile_contents(&columns)
    }
}

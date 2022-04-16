use crate::{FurColumn, FurTable};
use std::error::Error;

mod sortfile;
use sortfile::Sortfile;

impl FurTable {
    pub fn generate_sortfile_content(
        &mut self,
        column: &FurColumn,
    ) -> Result<Sortfile, Box<dyn Error>> {
        let rows = &self.get_all()?;
        let mut sortlist: Vec<u64> = (0..(rows.len() as u64)).collect();

        sortlist.sort_by(|a, b| {
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

        let current_sortfile = Sortfile::new(&column.get_id().clone(), &sortlist)?;

        Ok(current_sortfile)
    }

    pub fn generate_sortfile_contents(
        &mut self,
        columns: &[FurColumn],
    ) -> Result<(), Box<dyn Error>> {
        for column in columns {
            let current_sortfile = self.generate_sortfile_content(column)?;

            println!("{:?}", current_sortfile);

            current_sortfile.dump()?;
        }

        Ok(())
    }

    pub fn generate_all_sortfile_contents(&mut self) -> Result<(), Box<dyn Error>> {
        let columns = self.table_info.get_columns().clone();

        self.generate_sortfile_contents(&columns)
    }
}

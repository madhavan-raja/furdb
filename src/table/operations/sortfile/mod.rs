use crate::{FurColumn, FurTable};
use std::error::Error;

mod sortfile;
use sortfile::Sortfile;

impl FurTable {
    pub fn get_sortfile(&self, column: &FurColumn) -> Result<Sortfile, Box<dyn Error>> {
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

    pub fn generate_sortfile(&self, column: &FurColumn) -> Result<(), Box<dyn Error>> {
        let current_sortfile = self.get_sortfile(column)?;

        self.save_sortfile(&current_sortfile)
    }

    pub fn generate_sortfiles(&self, columns: &[FurColumn]) -> Result<(), Box<dyn Error>> {
        for column in columns {
            self.generate_sortfile(column)?;
        }

        Ok(())
    }

    pub fn generate_all_sortfiles(&mut self) -> Result<(), Box<dyn Error>> {
        let columns = self.table_info.get_columns().clone();

        self.generate_sortfiles(&columns)
    }

    pub(crate) fn save_sortfile(&self, sortfile_contents: &Sortfile) -> Result<(), Box<dyn Error>> {
        let column_id = sortfile_contents.get_column_id();
        let sortfile_contents = serde_json::to_string(sortfile_contents)?;
        let sortfile_path = Self::get_sortfile_path(&self.dir, &column_id);

        std::fs::write(sortfile_path, sortfile_contents)?;

        Ok(())
    }

    pub(crate) fn read_sortfile(&self, column_id: &str) -> Result<Sortfile, Box<dyn Error>> {
        let sortfile_path = Self::get_sortfile_path(&self.dir, column_id);
        let sortfile_contents_raw = std::fs::read_to_string(&sortfile_path)?;
        let sortfile_contents = serde_json::from_str(&sortfile_contents_raw)?;
        let sortfile = serde_json::from_value(sortfile_contents)?;

        Ok(sortfile)
    }

    pub fn clear_sortfile(&self, column: &FurColumn) -> Result<(), Box<dyn Error>> {
        let sortfile_path = Self::get_sortfile_path(&self.dir, &column.get_id().clone());
        std::fs::remove_file(sortfile_path)?;

        Ok(())
    }

    pub fn clear_sortfiles(&self, columns: &[FurColumn]) -> Result<(), Box<dyn Error>> {
        for column in columns {
            self.clear_sortfile(&column)?;
        }

        Ok(())
    }

    pub fn clear_all_sortfiles(&self) -> Result<(), Box<dyn Error>> {
        let columns = self.get_info()?.get_columns().clone();

        self.clear_sortfiles(&columns)
    }
}

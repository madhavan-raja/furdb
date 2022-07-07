use crate::{FurColumn, FurTable};
use std::{cmp::Ordering, error::Error};

mod sortfile;
use sortfile::Sortfile;

struct QuickSort {}

impl QuickSort {
    #![allow(unused_must_use)]
    pub async fn partition(
        tb: &FurTable,
        column: &FurColumn,
        vec: &mut Vec<u64>,
        low: i64,
        high: i64,
    ) -> Result<i64, Box<dyn Error>> {
        let mut i = low - 1;

        for j in low..(high - 1) {
            let bits_a = tb
                .get_row_bin(j as u64)?
                .get(&column.get_id())
                .unwrap()
                .clone();
            let bits_b = tb
                .get_row_bin(high as u64)?
                .get(&column.get_id())
                .unwrap()
                .clone();

            let data_type = column.get_data_type();
            let cmp = data_type
                .compare(&bits_a, &bits_b, tb.table_info.get_converter_server())
                .await?;

            if cmp == Ordering::Less {
                i = i + 1;
                vec.swap(i as usize, j as usize);
            }
        }
        vec.swap((i + 1) as usize, high as usize);

        Ok(i + 1)
    }

    pub async fn quicksort(
        tb: &FurTable,
        column: &FurColumn,
        vec: &mut Vec<u64>,
        low: i64,
        high: i64,
    ) -> Result<(), Box<dyn Error>> {
        let pi = Self::partition(tb, column, vec, low, high).await?;

        Self::quicksort(tb, column, vec, low, pi - 1);
        Self::quicksort(tb, column, vec, pi + 1, high);

        Ok(())
    }
}

impl FurTable {
    pub async fn get_sortfile(&self, column: &FurColumn) -> Result<Sortfile, Box<dyn Error>> {
        let row_count = Self::get_data_file_size(&self.dir)? / (self.get_row_size()? as u64 / 8);

        let mut sortlist: Vec<u64> = (0..(row_count)).collect();

        QuickSort::quicksort(&self, column, &mut sortlist, 0, row_count as i64 - 1).await?;

        let current_sortfile = Sortfile::new(&column.get_id().clone(), &sortlist)?;

        Ok(current_sortfile)
    }

    pub async fn generate_sortfile(&self, column: &FurColumn) -> Result<(), Box<dyn Error>> {
        let current_sortfile = self.get_sortfile(column).await?;

        self.save_sortfile(&current_sortfile)
    }

    pub async fn generate_sortfiles(&self, columns: &[FurColumn]) -> Result<(), Box<dyn Error>> {
        for column in columns {
            self.generate_sortfile(column).await?;
        }

        Ok(())
    }

    pub async fn generate_all_sortfiles(&mut self) -> Result<(), Box<dyn Error>> {
        let columns = self.table_info.get_columns().clone();

        self.generate_sortfiles(&columns).await
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
        if sortfile_path.exists() {
            std::fs::remove_file(sortfile_path)?;
        }

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

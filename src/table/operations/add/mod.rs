use crate::FurTable;
use bitvec::prelude::*;
use std::{collections::HashMap, error::Error};

mod utils;

impl FurTable {
    pub fn add(&mut self, rows: &[HashMap<&str, &str>]) -> Result<(), Box<dyn Error>> {
        let mut rows_bin = BitVec::<u8, Msb0>::new();

        for row in rows {
            let mut row_bin = self.convert_row_to_bin(row)?;
            rows_bin.append(&mut row_bin);
        }

        let bytes: Vec<u8> = rows_bin.into();
        self.write_data(&bytes)?;

        Ok(())
    }
}

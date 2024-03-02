use std::{fs::File, path::PathBuf};

use crate::TableInfo;

#[derive(Debug)]
pub struct Table {
    pub(crate) dir: PathBuf,
    pub(crate) data_file: File,
    pub(crate) data_file_size: u64,
    pub(crate) table_info: TableInfo,
}

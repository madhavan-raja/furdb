use std::path::PathBuf;

use crate::DatabaseInfo;

#[derive(Debug)]
pub struct Database {
    pub(crate) dir: PathBuf,
    pub(crate) database_info: DatabaseInfo,
}

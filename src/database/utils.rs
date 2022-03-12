use std::path::PathBuf;

use crate::{FurDB, FurDBInfo};

impl FurDB {
    pub(super) fn get_info_file_path(dir: &PathBuf) -> PathBuf {
        let mut db_info_file_path = dir.clone();
        db_info_file_path.push("fur.json");

        db_info_file_path
    }

    pub(super) fn ensure_db_files(
        dir: &PathBuf,
        db_info: Option<FurDBInfo>,
    ) -> std::io::Result<()> {
        if !dir.exists() {
            std::fs::create_dir(&dir)?;
        }

        let db_info_file_path = Self::get_info_file_path(&dir);

        if !db_info_file_path.exists() {
            let db_name = dir
                .file_name()
                .unwrap_or(std::ffi::OsStr::new(""))
                .to_str()
                .unwrap_or("");

            let db_info = db_info.unwrap_or(FurDBInfo::new(db_name));
            let db_info_contents_raw = serde_json::to_string(&db_info)?;

            std::fs::write(db_info_file_path, db_info_contents_raw)?;
        }

        Ok(())
    }
}

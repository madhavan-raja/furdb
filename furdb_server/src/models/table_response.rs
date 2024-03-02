use furdb_core::TableInfo;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct TableResponse {
    pub tb_info: TableInfo,
}

impl TableResponse {
    pub fn new(tb_info: TableInfo) -> Self {
        Self { tb_info }
    }
}

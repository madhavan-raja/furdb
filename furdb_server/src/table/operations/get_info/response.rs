use furdb_core::FurTableInfo;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct TableResponse {
    pub tb_info: FurTableInfo,
}

impl TableResponse {
    pub fn new(tb_info: FurTableInfo) -> Self {
        TableResponse { tb_info }
    }
}

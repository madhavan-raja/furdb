use crate::Column;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableInfo {
    pub(crate) name: String,
    pub(crate) converter_server: Option<String>,
    pub(crate) columns: Vec<Column>,
}

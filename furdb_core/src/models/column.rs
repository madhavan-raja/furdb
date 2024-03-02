use crate::DataType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Column {
    pub(crate) id: String,
    pub(crate) description: String,
    pub(crate) size: u128,
    pub(crate) data_type: DataType,
}

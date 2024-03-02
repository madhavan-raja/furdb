#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DataType {
    pub(crate) id: String,
    pub(crate) converter_endpoint_override: Option<String>,
}

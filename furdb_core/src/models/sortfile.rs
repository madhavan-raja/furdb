#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Sortfile {
    pub(crate) id: String,
    pub(crate) sortlist: Vec<u64>,
}

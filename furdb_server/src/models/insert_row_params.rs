#[derive(serde::Serialize, serde::Deserialize)]
pub struct InsertRowParams {
    pub(crate) data: Vec<u128>,
}

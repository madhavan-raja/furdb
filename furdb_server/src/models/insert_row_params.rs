use furdb_core::Column;

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct InsertRowParams {
    pub data: Vec<u128>,
}

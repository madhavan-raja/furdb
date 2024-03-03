use crate::models;

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct Query {
    pub column_id: String,
    pub value: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct QueryGeneratable {
    pub table_generatable: Option<models::CreateTableParams>,
    pub query: Option<Query>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct QueryGeneratable {
    pub table_generatable: Option<TableGenerator>,
    pub query: Option<Query>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct TableGenerator {
    pub name: String,
    pub converter_server: Option<String>,
    pub columns: Option<Vec<ColumnGenerator>>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct ColumnGenerator {
    pub id: String,
    pub description: Option<String>,
    pub size: u128,
    pub data_type: DataTypeGenerator,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct DataTypeGenerator {
    pub id: String,
    pub converter_endpoint_override: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct Query {
    pub column_id: String,
    pub value: String,
}

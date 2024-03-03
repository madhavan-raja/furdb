use crate::Column;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Table {
    pub(crate) table_id: String,
    pub(crate) table_name: String,
    pub(crate) table_columns: Vec<Column>,
}

impl Table {
    pub fn get_table_id(&self) -> String {
        self.table_id.clone()
    }

    pub fn get_table_name(&self) -> String {
        self.table_name.clone()
    }

    pub fn get_table_columns(&self) -> Vec<Column> {
        self.table_columns.clone()
    }
}

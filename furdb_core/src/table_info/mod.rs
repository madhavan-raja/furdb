use super::column::FurColumn;
use std::error::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TableInfo {
    name: String,
    converter_server: Option<String>,
    columns: Vec<FurColumn>,
}

impl TableInfo {
    pub fn new(
        name: &str,
        converter_server: Option<&str>,
        columns: Option<Vec<FurColumn>>,
    ) -> Result<Self, Box<dyn Error>> {
        let name = String::from(name);
        let converter_server = converter_server.map(str::to_string);
        let columns = columns.unwrap_or(Vec::new());

        if !Self::is_size_valid(&columns) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Size of the row should be a multiple of 8.",
            )));
        }

        Ok(Self {
            name,
            converter_server,
            columns,
        })
    }

    fn is_size_valid(columns: &[FurColumn]) -> bool {
        let mut row_size: u128 = 0;

        for column in columns {
            row_size += column.get_size();
        }

        row_size % 8 == 0
    }

    pub fn get_converter_server(&self) -> Option<String> {
        self.converter_server.clone()
    }

    pub fn get_columns(&self) -> &Vec<FurColumn> {
        &self.columns
    }
}

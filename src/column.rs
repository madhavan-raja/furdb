use super::FurDataType;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FurColumn {
    id: String,
    description: String,
    size: u128,
    data_type: FurDataType,
}

impl FurColumn {
    pub fn new(
        id: &str,
        description: Option<&str>,
        size: u128,
        data_type: FurDataType,
    ) -> FurColumn {
        FurColumn {
            id: String::from(id),
            description: String::from(description.unwrap_or(&id)),
            size,
            data_type: data_type,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_size(&self) -> u128 {
        self.size
    }

    pub fn get_data_type(&self) -> &FurDataType {
        &self.data_type
    }
}

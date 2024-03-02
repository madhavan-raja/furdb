use crate::{Column, DataType};

impl Column {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_size(&self) -> u128 {
        self.size
    }

    pub fn get_data_type(&self) -> DataType {
        self.data_type.clone()
    }
}

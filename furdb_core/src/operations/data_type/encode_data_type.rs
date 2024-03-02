use std::error::Error;

use crate::DataType;
use bitvec::prelude::*;

impl DataType {
    pub async fn encode(
        &self,
        data: &str,
        size: u128,
        converter_server: Option<String>,
    ) -> Result<BitVec<u8, Msb0>, Box<dyn Error>> {
        let data: String = data.into();
        let converter_endpoint = self.get_converter(converter_server);
        let url = format!("{}/encode?data={}&size={}", converter_endpoint, data, size);
        let res = reqwest::get(url).await?.text().await?;
        let binary = Self::string_to_bitvec(&res);

        Ok(binary)
    }
}

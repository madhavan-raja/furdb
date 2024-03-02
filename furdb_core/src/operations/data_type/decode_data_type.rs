use std::error::Error;

use crate::DataType;
use bitvec::prelude::*;

impl DataType {
    pub async fn decode(
        &self,
        bits: &BitVec<u8, Msb0>,
        converter_server: Option<String>,
    ) -> Result<String, Box<dyn Error>> {
        let bits = Self::bitvec_to_string(bits);
        let converter_endpoint = self.get_converter(converter_server);
        let url = format!("{}/decode?binary={}", converter_endpoint, bits);
        let res = reqwest::get(url).await?.text().await?;

        Ok(res)
    }
}

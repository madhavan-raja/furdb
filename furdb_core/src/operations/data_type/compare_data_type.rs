use crate::DataType;
use bitvec::prelude::*;
use std::error::Error;

impl DataType {
    pub async fn compare(
        &self,
        bits_a: &BitVec<u8, Msb0>,
        bits_b: &BitVec<u8, Msb0>,
        converter_server: Option<String>,
    ) -> Result<std::cmp::Ordering, Box<dyn Error>> {
        let bits_a = Self::bitvec_to_string(bits_a);
        let bits_b = Self::bitvec_to_string(bits_b);
        let converter_endpoint = self.get_converter(converter_server);
        let url = format!("{}/compare?a={}&b={}", converter_endpoint, bits_a, bits_b);
        let res = reqwest::get(url).await?.text().await?;

        let cmp: std::cmp::Ordering;

        match res.as_str() {
            "-1" => cmp = std::cmp::Ordering::Greater,
            "1" => cmp = std::cmp::Ordering::Less,
            _ => cmp = std::cmp::Ordering::Equal,
        }

        Ok(cmp)
    }
}

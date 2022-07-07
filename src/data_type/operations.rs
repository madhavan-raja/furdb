use crate::FurDataType;
use bitvec::prelude::*;
use std::error::Error;

impl FurDataType {
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

    pub fn compare(
        &self,
        bits_a: &BitVec<u8, Msb0>,
        bits_b: &BitVec<u8, Msb0>,
        converter_server: Option<String>,
    ) -> Result<std::cmp::Ordering, Box<dyn Error>> {
        let bits_a = Self::bitvec_to_string(bits_a);
        let bits_b = Self::bitvec_to_string(bits_b);
        let converter_endpoint = self.get_converter(converter_server);
        let url = format!("{}/compare?a={}&b={}", converter_endpoint, bits_a, bits_b);
        let res = reqwest::blocking::get(url)?.text()?;

        let cmp: std::cmp::Ordering;

        match res.as_str() {
            "-1" => cmp = std::cmp::Ordering::Greater,
            "1" => cmp = std::cmp::Ordering::Less,
            _ => cmp = std::cmp::Ordering::Equal,
        }

        Ok(cmp)
    }
}

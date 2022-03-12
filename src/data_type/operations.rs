use crate::FurDataType;
use bitvec::prelude::*;
use std::error::Error;

impl FurDataType {
    pub fn encode(
        &self,
        data: &str,
        size: u128,
        converter_server: Option<String>,
    ) -> Result<BitVec<u8, Msb0>, Box<dyn Error>> {
        let data: String = data.into();

        let converter_endpoint = self.get_converter(converter_server);

        let url = format!("{}/encode?data={}&size={}", converter_endpoint, data, size);

        let res = reqwest::blocking::get(url)?.text()?;

        let binary = Self::string_to_bitvec(&res);

        Ok(binary)
    }

    pub fn decode(
        &self,
        bits: &BitVec<u8, Msb0>,
        converter_server: Option<String>,
    ) -> Result<String, Box<dyn Error>> {
        let bits = Self::bitvec_to_string(bits);

        let converter_endpoint = self.get_converter(converter_server);

        let url = format!("{}/decode?binary={}", converter_endpoint, bits);

        let res = reqwest::blocking::get(url)?.text()?;

        Ok(res)
    }
}

use crate::FurDataType;
use bitvec::prelude::*;

impl FurDataType {
    pub(super) fn string_to_bitvec(bits: &String) -> BitVec<u8, Msb0> {
        let mut binary = BitVec::<u8, Msb0>::new();

        for bit in bits.chars() {
            binary.push(bit == '1');
        }

        binary
    }

    pub(super) fn bitvec_to_string(bits: &BitVec<u8, Msb0>) -> String {
        let mut binary = String::new();

        for bit in bits {
            binary.push(if *bit { '1' } else { '0' });
        }

        binary
    }

    pub(super) fn get_converter(&self, converter_server: Option<String>) -> String {
        let converter_endpoint = if self.converter_endpoint_override.is_some() {
            self.converter_endpoint_override.clone().unwrap()
        } else {
            format!("{}/{}", converter_server.unwrap(), self.id)
        };

        converter_endpoint
    }
}

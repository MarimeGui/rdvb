use crate::mpeg::{ServiceType, decode_stupid_string};

pub const DESCRIPTOR_ID: u8 = 0x48;

#[derive(Debug, Clone)]
pub struct Service {
    pub service_type: ServiceType,
    pub provider: String,
    pub service: String,
}

impl Service {
    pub fn from_buf(buf: &[u8]) -> Service {
        // TODO: Enum for these types
        let service_type = ServiceType::from_byte(buf[0]);

        let mut pos = 1;

        // Read provider string
        let provider_length = buf[pos];
        pos += 1;
        let raw_provider = &buf[pos..pos + provider_length as usize];
        pos += provider_length as usize;

        // Read service string
        let service_length = buf[pos];
        pos += 1;
        let raw_service = &buf[pos..pos + service_length as usize];
        // pos += service_length as usize;

        // TODO: Proper decoding (ETSI EN 300 468 page 135)
        let provider = decode_stupid_string(raw_provider).unwrap();
        let service = decode_stupid_string(raw_service).unwrap();

        Service {
            service_type,
            provider,
            service,
        }
    }
}

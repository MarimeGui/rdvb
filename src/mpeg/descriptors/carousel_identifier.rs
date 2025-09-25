pub const DESCRIPTOR_ID: u8 = 0x13;

// ETSI TS 102 809 page 125
#[derive(Debug, Clone)]
pub struct CarouselIdentifier {
    pub carousel_id: u32,
    pub identifier: Identifier,
}

#[derive(Debug, Clone)]
pub enum Identifier {
    Standard {
        private_data_bytes: Vec<u8>,
    },
    Enhanced {
        module_version: u8,
        module_id: u16,
        block_size: u16,
        module_size: u32,
        compression_method: u8,
        original_size: u32,
        time_out: u8,
        object_key_length: u8,
        object_key_data: Vec<u8>,
        private_data_byte: Vec<u8>,
    },
}

impl CarouselIdentifier {
    pub fn from_buf(buf: &[u8]) -> CarouselIdentifier {
        let carousel_id = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let format_id = buf[4];

        let identifier = if format_id == 0 {
            let private_data_bytes = buf[5..].to_vec();
            Identifier::Standard { private_data_bytes }
        } else if format_id == 1 {
            let module_version = buf[5];
            let module_id = u16::from_be_bytes([buf[6], buf[7]]);
            let block_size = u16::from_be_bytes([buf[8], buf[9]]);
            let module_size = u32::from_be_bytes([buf[10], buf[11], buf[12], buf[13]]);
            let compression_method = buf[14];
            let original_size = u32::from_be_bytes([buf[15], buf[16], buf[17], buf[18]]);
            let time_out = buf[19];
            let object_key_length = buf[20];
            let object_key_data = buf[21..21 + object_key_length as usize].to_vec();
            let private_data_byte = buf[21 + object_key_length as usize..].to_vec();

            Identifier::Enhanced {
                module_version,
                module_id,
                block_size,
                module_size,
                compression_method,
                original_size,
                time_out,
                object_key_length,
                object_key_data,
                private_data_byte,
            }
        } else {
            panic!(
                "unexpected format id ({}) for CarouselIdentifier descriptor",
                format_id
            )
        };

        CarouselIdentifier {
            carousel_id,
            identifier,
        }
    }
}

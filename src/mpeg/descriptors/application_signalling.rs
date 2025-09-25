pub const DESCRIPTOR_ID: u8 = 0x6F;

// ETSI TS 102 809 page 37
#[derive(Debug, Clone)]
pub struct ApplicationSignalling {
    pub elements: Vec<ApplicationSignallingElement>,
}

#[derive(Debug, Clone)]
pub struct ApplicationSignallingElement {
    pub application_type: u16,
    pub ait_version_number: u8,
}

impl ApplicationSignalling {
    pub fn from_buf(buf: &[u8]) -> ApplicationSignalling {
        let mut elements = Vec::new();

        let mut offset = 0;
        while offset < buf.len() {
            let _reserved = (buf[offset] & 0b1000_0000) != 0;
            let application_type = u16::from_be_bytes([buf[offset] & 0b0111_1111, buf[offset + 1]]);
            offset += 2;
            let _reserved = buf[offset] & 0b1110_0000;
            let ait_version_number = buf[offset] & 0b0001_1111;
            offset += 1;
            elements.push(ApplicationSignallingElement {
                application_type,
                ait_version_number,
            });
        }

        ApplicationSignalling { elements }
    }
}

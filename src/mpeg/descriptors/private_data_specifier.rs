pub const DESCRIPTOR_ID: u8 = 0x5F;

#[derive(Debug, Clone)]
pub struct PrivateDataSpecifier {
    pub specifier: u32,
}

impl PrivateDataSpecifier {
    pub fn from_buf(buf: &[u8]) -> PrivateDataSpecifier {
        PrivateDataSpecifier {
            specifier: u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]),
        }
    }
}

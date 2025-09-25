pub const DESCRIPTOR_ID: u8 = 0x40;

#[derive(Debug, Clone)]
pub struct NetworkName {
    // TODO: Should have a dedicated String type for this weird strings
    pub name: Vec<u8>,
}

impl NetworkName {
    pub fn from_buf(buf: &[u8]) -> NetworkName {
        NetworkName { name: buf.to_vec() }
    }
}

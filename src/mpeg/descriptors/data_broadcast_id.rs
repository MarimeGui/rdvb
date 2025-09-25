pub const DESCRIPTOR_ID: u8 = 0x66;

// ETSI EN 300 468 page 57
#[derive(Debug, Clone)]
pub struct DataBroadcastId {
    pub data_broadcast_id: u16,
    pub selector_bytes: Vec<u8>,
}

impl DataBroadcastId {
    pub fn from_buf(buf: &[u8]) -> DataBroadcastId {
        let data_broadcast_id = u16::from_be_bytes([buf[0], buf[1]]);
        let selector_bytes = buf[2..].to_vec();

        DataBroadcastId {
            data_broadcast_id,
            selector_bytes,
        }
    }
}

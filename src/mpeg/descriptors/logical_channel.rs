pub const DESCRIPTOR_ID: u8 = 0x83;

// According to docs, this is "user-defined"... Where are LCN descriptors "officially" defined ???

// w_scan2
#[derive(Debug, Clone)]
pub struct LogicalChannel {
    pub elements: Vec<LogicalChannelDescriptorElement>,
}

#[derive(Debug, Clone)]
pub struct LogicalChannelDescriptorElement {
    pub service_id: u16,
    pub visible_service: bool,
    pub logical_channel_number: u16,
}

impl LogicalChannel {
    pub fn from_buf(buf: &[u8]) -> LogicalChannel {
        let mut elements = Vec::new();

        let mut offset = 0;

        while offset < buf.len() {
            let service_id = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
            let visible_service = (buf[offset + 2] & 0b1000_0000) != 0;
            let logical_channel_number =
                u16::from_be_bytes([buf[offset + 2] & 0b0000_0011, buf[offset + 3]]);

            offset += 4;

            elements.push(LogicalChannelDescriptorElement {
                service_id,
                visible_service,
                logical_channel_number,
            });
        }

        LogicalChannel { elements }
    }
}

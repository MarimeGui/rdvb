pub const DESCRIPTOR_ID: u8 = 0x50;

// ETSI EN 300 468 page 45
#[derive(Debug, Clone)]
pub struct Component {
    pub stream_content_ext: u8,
    pub stream_content: u8,
    pub component_type: u8,
    pub component_tag: u8,
    pub language_code: [u8; 3],
    pub chars: Vec<u8>,
}

impl Component {
    pub fn from_buf(buf: &[u8]) -> Component {
        let stream_content_ext = buf[0] & 0b1111_0000;
        let stream_content = buf[0] & 0b0000_1111;
        let component_type = buf[1];
        let component_tag = buf[2];
        let language_code = [buf[3], buf[4], buf[5]];
        let chars = buf[6..].to_vec();

        Component {
            stream_content_ext,
            stream_content,
            component_type,
            component_tag,
            language_code,
            chars,
        }
    }
}

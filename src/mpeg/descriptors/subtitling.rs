pub const DESCRIPTOR_ID: u8 = 0x59;

// ETSI EN 300 468 page 91
#[derive(Debug, Clone)]
pub struct Subtitling {
    pub elements: Vec<SubtitlingElement>,
}

#[derive(Debug, Clone)]
pub struct SubtitlingElement {
    // ISO 639
    pub language_code: [u8; 3],
    pub subtitling_type: u8,
    pub composition_page_id: u16,
    pub ancillary_page_id: u16,
}

impl Subtitling {
    pub fn from_buf(buf: &[u8]) -> Subtitling {
        let mut elements = Vec::new();

        let mut offset = 0;
        while offset < buf.len() {
            let language_code = [buf[offset], buf[offset + 1], buf[offset + 2]];
            offset += 3;
            let subtitling_type = buf[offset];
            offset += 1;
            let composition_page_id = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
            offset += 2;
            let ancillary_page_id = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
            offset += 2;
            elements.push(SubtitlingElement {
                language_code,
                subtitling_type,
                composition_page_id,
                ancillary_page_id,
            })
        }

        Subtitling { elements }
    }
}

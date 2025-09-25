pub const DESCRIPTOR_ID: u8 = 0x7F;

// ETSI EN 300 468 page 65
// TODO: There may be more to this according to w_scan2
#[derive(Debug, Clone)]
pub struct Extension {
    pub selector_bytes: Vec<u8>,
}

impl Extension {
    pub fn from_buf(buf: &[u8]) -> Extension {
        //let tag_extension = buf[0];
        let selector_bytes = buf[1..].to_vec();

        Extension { selector_bytes }
    }
}

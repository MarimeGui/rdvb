pub const DESCRIPTOR_ID: u8 = 0x0A;

#[derive(Debug, Clone)]
pub struct Iso639Language {
    pub language: [u8; 4],
}

impl Iso639Language {
    pub fn from_buf(buf: &[u8]) -> Iso639Language {
        if buf.len() != 4 {
            // TODO: Error
            panic!()
        }

        Self {
            language: [buf[0], buf[1], buf[2], buf[3]],
        }
    }
}

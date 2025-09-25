pub const DESCRIPTOR_ID: u8 = 0x6A;

#[derive(Debug, Clone)]
pub struct Ac3 {
    pub component_type: Option<u8>,
    pub bsid: Option<u8>,
    pub mainid: Option<u8>,
    pub asvc: Option<u8>,
    pub additional_info_byte: Vec<u8>,
}

impl Ac3 {
    pub fn from_buf(buf: &[u8]) -> Ac3 {
        let component_type_flag = (buf[0] & 0b1000_0000) != 0;
        let bsid_flag = (buf[0] & 0b0100_0000) != 0;
        let mainid_flag = (buf[0] & 0b0010_0000) != 0;
        let asvc_flag = (buf[0] & 0b0001_0000) != 0;
        let _reserved = (buf[0] & 0b0000_1111) != 0;

        let mut offset = 1;

        let component_type = if component_type_flag {
            let r = Some(buf[offset]);
            offset += 1;
            r
        } else {
            None
        };

        let bsid = if bsid_flag {
            let r = Some(buf[offset]);
            offset += 1;
            r
        } else {
            None
        };

        let mainid = if mainid_flag {
            let r = Some(buf[offset]);
            offset += 1;
            r
        } else {
            None
        };

        let asvc = if asvc_flag {
            let r = Some(buf[offset]);
            offset += 1;
            r
        } else {
            None
        };

        let additional_info_byte = buf[offset..].to_vec();

        // TODO: Re-use some of the stuff used below for Enhanced AC3

        Ac3 {
            component_type,
            bsid,
            mainid,
            asvc,
            additional_info_byte,
        }
    }
}

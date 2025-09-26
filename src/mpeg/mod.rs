pub mod descriptors;

pub const DMX_CHECK_CRC: u32 = 1;
pub const DMX_ONESHOT: u32 = 2;
pub const DMX_IMMEDIATE_START: u32 = 4;

// TODO: 0x2000 does not work anymore for receiving all packets. Is there still a way to get the entire stream ? I think mpv might have something.

// -----

pub struct Packet {
    pub header: PacketHeader,
    pub data: Vec<u8>,
    pub crc: u32,
}

impl Packet {
    pub fn from_buf(buf: &[u8]) -> Packet {
        let header = PacketHeader::from_buf(buf);

        let payload_start = PacketHeader::LENGTH;
        let payload_end = buf.len() - (PacketHeader::LENGTH - 4); // Remove header and CRC32 from total size
        let data = buf[payload_start..payload_end].to_vec();

        // TODO: At least, I assume ? I couldn't match the CRC... Not sure if there is an init value, or if the CRC field should not be used ?
        let crc_start = buf.len() - 4;
        let crc = u32::from_be_bytes([
            buf[crc_start],
            buf[crc_start + 1],
            buf[crc_start + 2],
            buf[crc_start + 3],
        ]);

        Self { header, data, crc }
    }
}

#[derive(Debug)]
pub struct PacketHeader {
    pub table_id: u8,
    pub section_syntax_indicator: bool,
    pub section_length: u16,
    /// May be used differently depending on section.
    pub identifier: u16,
    pub version_number: u8,
    pub current_next_indicator: bool,
    pub section_number: u8,
    pub last_section_number: u8,
}

impl PacketHeader {
    pub const LENGTH: usize = 8;

    pub fn from_buf(buf: &[u8]) -> PacketHeader {
        if buf.len() < Self::LENGTH {
            panic!()
        }

        let table_id = buf[0];
        let section_syntax_indicator = (buf[1] & 0b1000_0000) != 0;
        // assert_eq!(buf[1] & 0b0100_0000, 0); // TODO: This bit seems to be set for NIT table
        let _reserved_1 = buf[1] & 0b0011_0000;
        assert_eq!(buf[1] & 0b0000_1100, 0);
        let section_length = u16::from_be_bytes([buf[1] & 0b0000_0011, buf[2]]);
        assert!(section_length <= 0x3FD);
        let transport_stream_id = u16::from_be_bytes([buf[3], buf[4]]);
        let _reserved_2 = buf[5] & 0b1100_0000;
        let version_number = buf[5] & 0b0011_1110;
        let current_next_indicator = (buf[5] & 0b0000_0001) != 0;
        let section_number = buf[6];
        let last_section_number = buf[7];

        PacketHeader {
            table_id,
            section_syntax_indicator,
            section_length,
            identifier: transport_stream_id,
            version_number,
            current_next_indicator,
            section_number,
            last_section_number,
        }
    }

    pub fn payload_len(&self) -> u16 {
        self.section_length - (5 + 4)
    }
}

//
// -----

/// Table of all possible service types.
///
/// Taken from ETSI EN 300 468 page 85 (table 89)
#[derive(Debug, Clone)]
pub enum ServiceType {
    DigitalTelevision,
    DigitalRadioSound,
    Teletext,
    NvodReference,
    NvodTimeShifted,
    Mosaic,
    FmRadio,
    DvbSrmService,
    AdvancedCodecDigitalRadioSound,
    H264Mosaic,
    DataBroadcast,
    CiReserved,
    RcsMap,
    RcsForwardLinkSignalling,
    DvbMultimediaHomePlatform,
    Mpeg2HdDigitalTelevision,
    H264SdDigitalTelevision,
    H264SdnvodTimeShifted,
    H264SdnvodReference,
    H264HdDigitalTelevision,
    H264HdnvodTimeShifted,
    H264HdnvodReference,
    H264FrameCompatiblePlanoStereoscopicHdDigitalTelevision,
    H264FrameCompatiblePlanoStereoscopicHdnvodTimeShifted,
    H264FrameCompatiblePlanoStereoscopicHdnvodReference,
    HevcDigitalTelevision,
    HevcUhdDigitalTelevision,
    UserDefined(u8),
    Reserved(u8),
}

impl ServiceType {
    pub fn from_byte(byte: u8) -> ServiceType {
        match byte {
            0x01 => Self::DigitalTelevision,
            0x02 => Self::DigitalRadioSound,
            0x03 => Self::Teletext,
            0x04 => Self::NvodReference,
            0x05 => Self::NvodTimeShifted,
            0x06 => Self::Mosaic,
            0x07 => Self::FmRadio,
            0x08 => Self::DvbSrmService,
            0x0A => Self::AdvancedCodecDigitalRadioSound,
            0x0B => Self::H264Mosaic,
            0x0C => Self::DataBroadcast,
            0x0D => Self::CiReserved,
            0x0E => Self::RcsMap,
            0x0F => Self::RcsForwardLinkSignalling,
            0x10 => Self::DvbMultimediaHomePlatform,
            0x11 => Self::Mpeg2HdDigitalTelevision,
            0x16 => Self::H264SdDigitalTelevision,
            0x17 => Self::H264SdnvodTimeShifted,
            0x18 => Self::H264SdnvodReference,
            0x19 => Self::H264HdDigitalTelevision,
            0x1A => Self::H264HdnvodTimeShifted,
            0x1B => Self::H264HdnvodReference,
            0x1C => Self::H264FrameCompatiblePlanoStereoscopicHdDigitalTelevision,
            0x1D => Self::H264FrameCompatiblePlanoStereoscopicHdnvodTimeShifted,
            0x1E => Self::H264FrameCompatiblePlanoStereoscopicHdnvodReference,
            0x1F => Self::HevcDigitalTelevision,
            0x20 => Self::HevcUhdDigitalTelevision,
            0x80..=0xFE => Self::UserDefined(byte),
            _ => Self::Reserved(byte),
        }
    }
}

//
// -----

pub fn decode_stupid_string(raw_text: &[u8]) -> Option<String> {
    // For now, just do best-effort conversion and remove weird characters
    let converted = String::from_utf8_lossy(raw_text)
        .into_owned()
        .trim_matches(|c: char| c.is_control())
        .to_string();
    // println!(
    //     "{}: {:?}",
    //     converted,
    //     converted.chars().map(|c| c.escape_unicode())
    // );
    Some(converted)

    // let encoding = if raw_text[0] < 0x20 {
    //     // First byte defines character coding table
    //     match raw_text[0] {
    //         0x01 => encoding_rs::ISO_8859_5,
    //         0x02 => encoding_rs::ISO_8859_6,
    //         0x03 => encoding_rs::ISO_8859_7,
    //         0x04 => encoding_rs::ISO_8859_8,
    //         0x05 => encoding_rs::WINDOWS_1254,
    //         0x06 => encoding_rs::ISO_8859_10,
    //         // 0x07 => encoding_rs::ISO_8859_11,
    //         // 0x08 => panic!(),
    //         0x09 => encoding_rs::ISO_8859_13,
    //         0x0A => encoding_rs::ISO_8859_14,
    //         0x0B => encoding_rs::ISO_8859_15,
    //         // 0x0C..0x0F => panic!(),
    //         _ => return None,
    //     }
    // } else {
    //     // The default encoding is ISO 6937, a multi-byte encoding conveniently not in the Encoding Standard, i.e. not in encoding_rs.
    //     // Use the most basic Latin encoding and hope for the best.
    //     encoding_rs::WINDOWS_1252
    // };

    // // TODO: Can't really do that as they're also putting some crap custom control chars for some reason

    // todo!()
}

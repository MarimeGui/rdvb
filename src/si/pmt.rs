use crate::mpeg::{Packet, descriptors::Descriptor};

#[derive(Debug)]
pub struct ProgramMapTable {
    pub program_number: u16,
    pub pcr_pid: u16,
    pub program_info_descriptors: Vec<Descriptor>,
    pub elementary_streams: Vec<ElementaryStream>,
}

#[derive(Debug)]
pub struct ElementaryStream {
    pub stream_type: StreamType,
    pub elementary_pid: u16,
    pub descriptors: Vec<Descriptor>,
}

// ISO/IEC 13818-1 page 66, descriptors.h stream_type enum
// Also, w_scan2 scan.c parse_pmt fn
// TODO: Rename to something simpler, use docs for full name
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StreamType {
    ItuTIsoIecReserved,
    IsoIec11172Video,
    ItuTRecH262IsoIec13818_2VideoOrIsoIec11172_2ConstrainedParameterVideoStream,
    IsoIec11172Audio,
    IsoIec13818_3Audio,
    ItuTRecH2220IsoIec13818_1PrivateSections,
    ItuTRecH2220IsoIec13818_1PESPacketsContainingPrivateData,
    IsoIec13522Mheg,
    ItuTRecH2220IsoIec13818_1AnnexADsmCC,
    ItuTRecH2221,
    IsoIec13818_6TypeA,
    IsoIec13818_6TypeB,
    IsoIec13818_6TypeC,
    IsoIec13818_6TypeD,
    ItuTRecH2220IsoIec13818_1Auxiliary,
    IsoIec13818_7AudioWithAdtsTransportSyntax,
    IsoIec14496_2Visual,
    IsoIec14496_3AudioWithTheLatmTransportSyntaxAsDefinedInIsoIec14496_3Amd1,
    IsoIec14496_1SlPacketizedStreamOrFlexMuxStreamCarriedInPesPackets,
    IsoIec14496_1SlPacketizedStreamOrFlexMusStreamCarriedInIsoIec14496Sections,
    IsoIec13818_6SynchronizedDownloadProtocol,
    IsoIec14496_10AVCVideo,
    IsoIec23008_2H265,
    ItuTRecH2220IsoIec13818_1Reserved(u8),
    UserPrivate(u8),
}

impl StreamType {
    pub fn from_u8(value: u8) -> StreamType {
        match value {
            0x00 => Self::ItuTIsoIecReserved,
            0x01 => Self::IsoIec11172Video,
            0x02 => {
                Self::ItuTRecH262IsoIec13818_2VideoOrIsoIec11172_2ConstrainedParameterVideoStream
            }
            0x03 => Self::IsoIec11172Audio,
            0x04 => Self::IsoIec13818_3Audio,
            0x05 => Self::ItuTRecH2220IsoIec13818_1PrivateSections,
            0x06 => Self::ItuTRecH2220IsoIec13818_1PESPacketsContainingPrivateData,
            0x07 => Self::IsoIec13522Mheg,
            0x08 => Self::ItuTRecH2220IsoIec13818_1AnnexADsmCC,
            0x09 => Self::ItuTRecH2221,
            0x0A => Self::IsoIec13818_6TypeA,
            0x0B => Self::IsoIec13818_6TypeB,
            0x0C => Self::IsoIec13818_6TypeC,
            0x0D => Self::IsoIec13818_6TypeD,
            0x0E => Self::ItuTRecH2220IsoIec13818_1Auxiliary,
            0x0F => Self::IsoIec13818_7AudioWithAdtsTransportSyntax,
            0x10 => Self::IsoIec14496_2Visual,
            0x11 => Self::IsoIec14496_3AudioWithTheLatmTransportSyntaxAsDefinedInIsoIec14496_3Amd1,
            0x12 => Self::IsoIec14496_1SlPacketizedStreamOrFlexMuxStreamCarriedInPesPackets,
            0x13 => {
                Self::IsoIec14496_1SlPacketizedStreamOrFlexMusStreamCarriedInIsoIec14496Sections
            }
            0x14 => Self::IsoIec13818_6SynchronizedDownloadProtocol,
            0x1B => Self::IsoIec14496_10AVCVideo,
            0x24 => Self::IsoIec23008_2H265,
            0x15..0x7F => Self::ItuTRecH2220IsoIec13818_1Reserved(value),
            _ => Self::UserPrivate(value),
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            StreamType::ItuTIsoIecReserved => 0x00,
            StreamType::IsoIec11172Video => 0x01,
            StreamType::ItuTRecH262IsoIec13818_2VideoOrIsoIec11172_2ConstrainedParameterVideoStream => 0x02,
            StreamType::IsoIec11172Audio => 0x03,
            StreamType::IsoIec13818_3Audio => 0x04,
            StreamType::ItuTRecH2220IsoIec13818_1PrivateSections => 0x05,
            StreamType::ItuTRecH2220IsoIec13818_1PESPacketsContainingPrivateData => 0x06,
            StreamType::IsoIec13522Mheg => 0x07,
            StreamType::ItuTRecH2220IsoIec13818_1AnnexADsmCC => 0x08,
            StreamType::ItuTRecH2221 => 0x09,
            StreamType::IsoIec13818_6TypeA => 0x0A,
            StreamType::IsoIec13818_6TypeB => 0x0B,
            StreamType::IsoIec13818_6TypeC => 0x0C,
            StreamType::IsoIec13818_6TypeD => 0x0D,
            StreamType::ItuTRecH2220IsoIec13818_1Auxiliary => 0x0E,
            StreamType::IsoIec13818_7AudioWithAdtsTransportSyntax => 0x0F,
            StreamType::IsoIec14496_2Visual => 0x10,
            StreamType::IsoIec14496_3AudioWithTheLatmTransportSyntaxAsDefinedInIsoIec14496_3Amd1 => 0x11,
            StreamType::IsoIec14496_1SlPacketizedStreamOrFlexMuxStreamCarriedInPesPackets => 0x12,
            StreamType::IsoIec14496_1SlPacketizedStreamOrFlexMusStreamCarriedInIsoIec14496Sections => 0x13,
            StreamType::IsoIec13818_6SynchronizedDownloadProtocol => 0x14,
            StreamType::IsoIec14496_10AVCVideo => 0x1B,
            StreamType::IsoIec23008_2H265 => 0x24,
            StreamType::ItuTRecH2220IsoIec13818_1Reserved(x) => x,
            StreamType::UserPrivate(x) => x,
        }
    }

    pub fn is_video(self) -> bool {
        match self {
            StreamType::IsoIec11172Video => {}
            StreamType::ItuTRecH262IsoIec13818_2VideoOrIsoIec11172_2ConstrainedParameterVideoStream => {}
            StreamType::IsoIec14496_10AVCVideo => {}
            StreamType::IsoIec23008_2H265 => {}
            _ => return false
        }
        true
    }
}

// ISO/IEC 13818-1 page 64
pub fn parse_pmt(packet: &Packet) -> ProgramMapTable {
    let _reserved_1 = packet.data[0] & 0b1110_0000;
    let pcr_pid = u16::from_be_bytes([packet.data[0] & 0b0001_1111, packet.data[1]]);
    let _reserved_2 = packet.data[2] & 0b1111_0000;
    assert_eq!((packet.data[2] as u16) & 0b0000_1100, 0);
    let program_info_length = u16::from_be_bytes([packet.data[2] & 0b0000_0011, packet.data[3]]);

    // Parse descriptors
    // TODO: Not sure what these descriptors may contain as I've never seen any here
    let mut current_offset = 4;
    let program_info_descriptors = Descriptor::read_many(
        &packet.data[current_offset..current_offset + program_info_length as usize],
    );
    current_offset += program_info_length as usize;

    let mut elementary_streams = Vec::new();

    while (current_offset as u16) < packet.header.payload_len() {
        let stream_type = packet.data[current_offset];
        current_offset += 1;

        let _reserved_a = packet.data[current_offset] & 0b1110_0000;
        let elementary_pid = u16::from_be_bytes([
            packet.data[current_offset] & 0b0001_1111,
            packet.data[current_offset + 1],
        ]);
        current_offset += 2;

        let _reserved_b = packet.data[current_offset] & 0b1111_0000;
        assert_eq!((packet.data[current_offset] as u16) & 0b0000_1100, 0);
        let es_info_length = u16::from_be_bytes([
            packet.data[current_offset] & 0b0000_0011,
            packet.data[current_offset + 1],
        ]);
        current_offset += 2;

        let descriptors = Descriptor::read_many(
            &packet.data[current_offset..current_offset + es_info_length as usize],
        );
        current_offset += es_info_length as usize;

        elementary_streams.push(ElementaryStream {
            stream_type: StreamType::from_u8(stream_type),
            elementary_pid,
            descriptors,
        });
    }

    ProgramMapTable {
        program_number: packet.header.identifier,
        pcr_pid,
        program_info_descriptors,
        elementary_streams,
    }
}

use std::{path::Path, time::Duration};

use crate::demux::{
    Demux,
    sys::{DmxFilter, DmxSctFilterParams},
};

pub const DMX_CHECK_CRC: u32 = 1;
pub const DMX_ONESHOT: u32 = 2;
pub const DMX_IMMEDIATE_START: u32 = 4;

pub const PID_PAT: u16 = 0;
pub const PID_SDT_BAT_ST: u16 = 0x11;

/// Program Association Section
pub const TABLE_PAT: u16 = 0;
/// Program Map Section
pub const TABLE_PMT: u16 = 2;
/// Network Information Section - Actual network
pub const TABLE_NIT_ACT: u16 = 0x40;
/// Service Description Section - Actual transport stream
pub const TABLE_SDT_ACT: u16 = 0x42;

// TODO: 0x2000 does not work anymore for receiving all packets. Is there still a way to get the entire stream ? I think mpv might have something.

// -----

pub struct Packet {
    pub header: PacketHeader,
    pub data: Vec<u8>,
    pub crc: u32,
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
    const BUF_LEN: usize = 8;

    pub fn from_buf(buf: &[u8]) -> PacketHeader {
        if buf.len() < Self::BUF_LEN {
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

// TODO Lib: Get one packet with trait for specific section ?

pub struct PidTableIdPair {
    pub pid: u16,
    // TODO: Make this option
    pub table_id: u16,
}

/// Receives a single packet from specified PIDs and Table IDs
pub fn receive_single_packet_many(
    demux_path: &Path,
    pids_table_ids: Vec<PidTableIdPair>,
    timeout: Option<Duration>,
) -> Result<Vec<Packet>, std::io::Error> {
    // First, setup all demuxers for all requested pairs
    let mut demuxers = Vec::new();
    for pair in pids_table_ids {
        let mut demux = Demux::new(demux_path)?;

        let inner_filter = DmxFilter {
            filter: {
                let mut filter = [0; 16];
                if pair.table_id < 0x100 {
                    filter[0] = pair.table_id as u8;
                }
                filter
            },
            mask: {
                let mut mask = [0; 16];
                if pair.table_id < 0x100 {
                    mask[0] = 0xFF;
                }
                mask
            },
            mode: [0; 16],
        };
        let filter = DmxSctFilterParams {
            pid: pair.pid,
            filter: inner_filter,
            timeout: timeout.map(|d| d.as_millis() as u32).unwrap_or(0),
            flags: DMX_CHECK_CRC | DMX_ONESHOT | DMX_IMMEDIATE_START, // TODO: Proper thing later
        };
        demux.set_filter(&filter);
        demuxers.push(demux);
    }

    // Now, the kernel will keep a single packet as it arrives, and we can block on reading all of them

    // Read all demuxers
    let mut packets = Vec::new();
    for mut demux in demuxers.into_iter() {
        let mut buf = vec![0; 4096];
        let read = demux.read(&mut buf)?;
        buf.truncate(read);

        let header = PacketHeader::from_buf(&buf);

        let payload_start = PacketHeader::BUF_LEN;
        let payload_end = buf.len() - (PacketHeader::BUF_LEN - 4); // Remove header and CRC32 from total size
        let data = buf[payload_start..payload_end].to_vec();

        // TODO: At least, I assume ? I couldn't match the CRC... Not sure if there is an init value, or if the CRC field should not be used ?
        let crc_start = buf.len() - 4;
        let crc = u32::from_be_bytes([
            buf[crc_start],
            buf[crc_start + 1],
            buf[crc_start + 2],
            buf[crc_start + 3],
        ]);

        packets.push(Packet { header, data, crc });
    }

    Ok(packets)
}

pub fn receive_single_packet(
    demux_path: &Path,
    pid: u16,
    table_id: u16,
    timeout: Option<Duration>,
) -> Result<Packet, std::io::Error> {
    let packets =
        receive_single_packet_many(demux_path, vec![PidTableIdPair { pid, table_id }], timeout)?;
    let p = packets.into_iter().next().unwrap();
    Ok(p)
}

//
// -----

// TODO: Move these into individual structs

// Also look in vdr si.h DescriptorTag enum
#[derive(Debug, Clone)]
pub enum Descriptor {
    NetworkName {
        // TODO: Should have a dedicated String type for this weird strings
        name: Vec<u8>,
    },
    ServiceList {
        services: Vec<ServiceListDescriptorElement>,
    },
    Service {
        service_type: ServiceType,
        provider: String,
        service: String,
    },
    StreamIdentifier {
        /// Identifies the component stream for associating it with a description given in a component descriptor.
        component_tag: u8,
    },
    TerrestrialDeliverySystem {
        center_frequency: i32,
        bandwidth: u8,
        priority: bool,
        time_slicing_indicator: bool,
        mpe_fec_indicator: bool,
        constellation: u8,
        hierarchy_information: u8,
        code_rate_hp_stream: u8,
        code_rate_lp_stream: u8,
        guard_interval: u8,
        transmission_mode: u8,
        other_frequency_flag: bool,
    },
    // w_scan2
    LogicalChannel {
        elements: Vec<LogicalChannelDescriptorElement>,
    },
    // ETSI EN 300 468 page 156
    EnhancedAc3 {
        mixinfoexists: bool,
        component_type: Option<EnhancedAc3ComponentType>,
        bsid: Option<u8>,
        mainid: Option<u8>,
        asvc: Option<u8>,
        substream1: Option<u8>,
        substream2: Option<u8>,
        substream3: Option<u8>,
        additional_info: Vec<u8>,
    },
    PrivateDataSpecifier {
        specifier: u32,
    },
    // ETSI EN 300 468 page 57
    DataBroadcastId {
        data_broadcast_id: u16,
        selector_bytes: Vec<u8>,
    },
    // ETSI EN 300 468 page 65
    // TODO: There may be more to this according to w_scan2
    Extension {
        selector_bytes: Vec<u8>,
    },
    // ETSI EN 300 468 page 91
    Subtitling {
        elements: Vec<SubtitlingElement>,
    },
    // ETSI EN 300 468 page 45
    Component {
        stream_content_ext: u8,
        stream_content: u8,
        component_type: u8,
        component_tag: u8,
        language_code: [u8; 3],
        chars: Vec<u8>,
    },
    Iso639Language {
        language: [u8; 4],
    },
    // ETSI TS 102 809 page 37
    ApplicationSignalling {
        elements: Vec<ApplicationSignallingElement>,
    },
    Ac3 {
        component_type: Option<u8>,
        bsid: Option<u8>,
        mainid: Option<u8>,
        asvc: Option<u8>,
        additional_info_byte: Vec<u8>,
    },
    // ETSI TS 102 809 page 125
    CarouselIdentifier {
        carousel_id: u32,
        identifier: CarouselIdentifier,
    },
    _Unknown {
        descriptor_id: u8,
        raw_data: Vec<u8>,
    },
}

#[derive(Debug, Clone)]
pub struct ServiceListDescriptorElement {
    /// Same as program number in program map except for 0x04, 0x18, 0x1B (NVOD services) (from ETSI EN 300 468)
    pub service_id: u16,
    pub service_type: ServiceType,
}

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

#[derive(Debug, Clone)]
pub struct LogicalChannelDescriptorElement {
    pub service_id: u16,
    pub visible_service: bool,
    pub logical_channel_number: u16,
}

#[derive(Debug, Clone)]
pub struct EnhancedAc3ComponentType {
    pub enhanced: bool,
    pub full_service: bool,
    pub service_type: EnhancedAc3ServiceType,
    pub channel_setup: EnhancedAc3ChannelSetup,
}

#[derive(Debug, Clone)]
pub enum EnhancedAc3ServiceType {
    CompleteMain,
    MusicAndEffects,
    VisuallyImpaired,
    HearingImpaired,
    Dialogue,
    Commentary,
    Emergency,
    Voiceover,
    Karaoke,
    _Invalid(bool, bool, bool),
}

#[derive(Debug, Clone)]
pub enum EnhancedAc3ChannelSetup {
    Mono,
    TwoIndependent,
    Stereo,
    SurroundStereoEncoded,
    MultichannelOver2,
    MultichannelOver5Dot1,
    Independent,
    Reserved,
}

#[derive(Debug, Clone)]
pub struct SubtitlingElement {
    // ISO 639
    pub language_code: [u8; 3],
    pub subtitling_type: u8,
    pub composition_page_id: u16,
    pub ancillary_page_id: u16,
}

#[derive(Debug, Clone)]
pub struct ApplicationSignallingElement {
    pub application_type: u16,
    pub ait_version_number: u8,
}

#[derive(Debug, Clone)]
pub enum CarouselIdentifier {
    Standard {
        private_data_bytes: Vec<u8>,
    },
    Enhanced {
        module_version: u8,
        module_id: u16,
        block_size: u16,
        module_size: u32,
        compression_method: u8,
        original_size: u32,
        time_out: u8,
        object_key_length: u8,
        object_key_data: Vec<u8>,
        private_data_byte: Vec<u8>,
    },
}

impl Descriptor {
    pub fn read(descriptor_id: u8, length: u8, buf: &[u8]) -> Descriptor {
        match descriptor_id {
            // 0x05 => {} // In TS 102 809, but does not correspond to the data I'm getting
            0x0A => {
                if length != 4 {
                    panic!("unexpected length for 0x0A descriptor ({} bytes)", length)
                }

                Descriptor::Iso639Language {
                    language: [buf[0], buf[1], buf[2], buf[3]],
                }
            }
            // 0x09 => {} // In TS 102 809, but does not correspond to the data I'm getting
            // 0x0E => {}, // Seen on a DVB-T2 HEVC channel
            0x13 => {
                let carousel_id = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
                let format_id = buf[4];

                let identifier = if format_id == 0 {
                    let private_data_bytes = buf[5..].to_vec();
                    CarouselIdentifier::Standard { private_data_bytes }
                } else if format_id == 1 {
                    let module_version = buf[5];
                    let module_id = u16::from_be_bytes([buf[6], buf[7]]);
                    let block_size = u16::from_be_bytes([buf[8], buf[9]]);
                    let module_size = u32::from_be_bytes([buf[10], buf[11], buf[12], buf[13]]);
                    let compression_method = buf[14];
                    let original_size = u32::from_be_bytes([buf[15], buf[16], buf[17], buf[18]]);
                    let time_out = buf[19];
                    let object_key_length = buf[20];
                    let object_key_data = buf[21..21 + object_key_length as usize].to_vec();
                    let private_data_byte = buf[21 + object_key_length as usize..].to_vec();

                    CarouselIdentifier::Enhanced {
                        module_version,
                        module_id,
                        block_size,
                        module_size,
                        compression_method,
                        original_size,
                        time_out,
                        object_key_length,
                        object_key_data,
                        private_data_byte,
                    }
                } else {
                    panic!(
                        "unexpected format id ({}) for CarouselIdentifier descriptor",
                        format_id
                    )
                };

                Descriptor::CarouselIdentifier {
                    carousel_id,
                    identifier,
                }
            }
            // 0x38 => {}, // Seen on a DVB-T2 HEVC channel
            0x40 => Descriptor::NetworkName { name: buf.to_vec() },
            0x41 => {
                let mut services = Vec::new();

                let mut offset = 0;
                while offset < length as usize {
                    let service_id = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
                    let service_type = ServiceType::from_byte(buf[offset + 2]);
                    offset += 3;
                    services.push(ServiceListDescriptorElement {
                        service_id,
                        service_type,
                    });
                }

                Descriptor::ServiceList { services }
            }
            0x48 => {
                // TODO: Enum for these types
                let service_type = ServiceType::from_byte(buf[0]);

                let mut pos = 1;

                // Read provider string
                let provider_length = buf[pos];
                pos += 1;
                let raw_provider = &buf[pos..pos + provider_length as usize];
                pos += provider_length as usize;

                // Read service string
                let service_length = buf[pos];
                pos += 1;
                let raw_service = &buf[pos..pos + service_length as usize];
                // pos += service_length as usize;

                // TODO: Proper decoding (ETSI EN 300 468 page 135)
                let provider = decode_stupid_string(raw_provider).unwrap();
                let service = decode_stupid_string(raw_service).unwrap();

                Descriptor::Service {
                    service_type,
                    provider,
                    service,
                }
            }
            0x52 => {
                let component_tag = buf[0];
                Descriptor::StreamIdentifier { component_tag }
            }
            0x50 => {
                let stream_content_ext = buf[0] & 0b1111_0000;
                let stream_content = buf[0] & 0b0000_1111;
                let component_type = buf[1];
                let component_tag = buf[2];
                let language_code = [buf[3], buf[4], buf[5]];
                let chars = buf[6..].to_vec();

                Descriptor::Component {
                    stream_content_ext,
                    stream_content,
                    component_type,
                    component_tag,
                    language_code,
                    chars,
                }
            }
            0x5A => {
                let center_frequency = i32::from_be_bytes([buf[0], buf[2], buf[1], buf[3]]);
                let bandwidth = (buf[4] & 0b1110_0000) >> 5;
                let priority = (buf[4] & 0b0001_0000) != 0;
                let time_slicing_indicator = (buf[4] & 0b0000_1000) != 0;
                let mpe_fec_indicator = (buf[4] & 0b0000_0100) != 0;
                let _reserved = buf[4] & 0b0000_0011;
                let constellation = (buf[5] & 0b1100_0000) >> 6;
                let hierarchy_information = (buf[5] & 0b0011_1000) >> 3;
                let code_rate_hp_stream = buf[5] & 0b0000_0111;
                let code_rate_lp_stream = (buf[6] & 0b1110_0000) >> 5;
                let guard_interval = (buf[6] & 0b0001_1000) >> 3;
                let transmission_mode = (buf[6] & 0b0000_0110) >> 1;
                let other_frequency_flag = (buf[6] & 0b0000_0001) != 0;
                let _reserved = u32::from_be_bytes([buf[7], buf[8], buf[9], buf[10]]);

                Descriptor::TerrestrialDeliverySystem {
                    center_frequency,
                    bandwidth,
                    priority,
                    time_slicing_indicator,
                    mpe_fec_indicator,
                    constellation,
                    hierarchy_information,
                    code_rate_hp_stream,
                    code_rate_lp_stream,
                    guard_interval,
                    transmission_mode,
                    other_frequency_flag,
                }
            }
            0x59 => {
                let mut elements = Vec::new();

                let mut offset = 0;
                while offset < length as usize {
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

                Descriptor::Subtitling { elements }
            }
            0x5F => Descriptor::PrivateDataSpecifier {
                specifier: u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]),
            },
            0x66 => {
                let data_broadcast_id = u16::from_be_bytes([buf[0], buf[1]]);
                let selector_bytes = buf[2..].to_vec();
                Descriptor::DataBroadcastId {
                    data_broadcast_id,
                    selector_bytes,
                }
            }
            0x6A => {
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

                Descriptor::Ac3 {
                    component_type,
                    bsid,
                    mainid,
                    asvc,
                    additional_info_byte,
                }
            }
            0x6F => {
                let mut elements = Vec::new();

                let mut offset = 0;
                while offset < length as usize {
                    let _reserved = (buf[offset] & 0b1000_0000) != 0;
                    let application_type =
                        u16::from_be_bytes([buf[offset] & 0b0111_1111, buf[offset + 1]]);
                    offset += 2;
                    let _reserved = buf[offset] & 0b1110_0000;
                    let ait_version_number = buf[offset] & 0b0001_1111;
                    offset += 1;
                    elements.push(ApplicationSignallingElement {
                        application_type,
                        ait_version_number,
                    });
                }

                Descriptor::ApplicationSignalling { elements }
            }
            0x7A => {
                let mut offset = 0;

                let component_type_flag = (buf[0] & 0b1000_0000) != 0;
                let bsid_flag = (buf[0] & 0b0100_0000) != 0;
                let mainid_flag = (buf[0] & 0b0010_0000) != 0;
                let asvc_flag = (buf[0] & 0b0001_0000) != 0;
                let mixinfoexists = (buf[0] & 0b0000_1000) != 0;
                let substream1_flag = (buf[0] & 0b0000_0100) != 0;
                let substream2_flag = (buf[0] & 0b0000_0010) != 0;
                let substream3_flag = (buf[0] & 0b0000_0001) != 0;
                offset += 1;

                let component_type = if component_type_flag {
                    let byte = buf[offset];
                    offset += 1;

                    let full_service = (byte & 0b0100_0000) != 0;

                    Some(EnhancedAc3ComponentType {
                        enhanced: (byte & 0b1000_0000) != 0,
                        full_service,
                        service_type: match (
                            (byte & 0b0010_0000) != 0,
                            (byte & 0b0001_0000) != 0,
                            (byte & 0b0000_1000) != 0,
                            full_service,
                        ) {
                            (false, false, false, true) => EnhancedAc3ServiceType::CompleteMain,
                            (false, false, true, false) => EnhancedAc3ServiceType::MusicAndEffects,
                            (false, true, false, _) => EnhancedAc3ServiceType::VisuallyImpaired,
                            (false, true, true, _) => EnhancedAc3ServiceType::HearingImpaired,
                            (true, false, false, false) => EnhancedAc3ServiceType::Dialogue,
                            (true, false, true, _) => EnhancedAc3ServiceType::Commentary,
                            (true, true, false, true) => EnhancedAc3ServiceType::Emergency,
                            (true, true, true, false) => EnhancedAc3ServiceType::Voiceover,
                            (true, true, true, true) => EnhancedAc3ServiceType::Karaoke,
                            (x, y, z, _) => EnhancedAc3ServiceType::_Invalid(x, y, z),
                        },
                        channel_setup: match (
                            (byte & 0b0010_0000) != 0,
                            (byte & 0b0001_0000) != 0,
                            (byte & 0b0000_1000) != 0,
                        ) {
                            (false, false, false) => EnhancedAc3ChannelSetup::Mono,
                            (false, false, true) => EnhancedAc3ChannelSetup::TwoIndependent,
                            (false, true, false) => EnhancedAc3ChannelSetup::Stereo,
                            (false, true, true) => EnhancedAc3ChannelSetup::SurroundStereoEncoded,
                            (true, false, false) => EnhancedAc3ChannelSetup::MultichannelOver2,
                            (true, false, true) => EnhancedAc3ChannelSetup::MultichannelOver5Dot1,
                            (true, true, false) => EnhancedAc3ChannelSetup::Independent,
                            (true, true, true) => EnhancedAc3ChannelSetup::Reserved,
                        },
                    })
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

                let substream1 = if substream1_flag {
                    let r = Some(buf[offset]);
                    offset += 1;
                    r
                } else {
                    None
                };

                let substream2 = if substream2_flag {
                    let r = Some(buf[offset]);
                    offset += 1;
                    r
                } else {
                    None
                };

                let substream3 = if substream3_flag {
                    let r = Some(buf[offset]);
                    offset += 1;
                    r
                } else {
                    None
                };

                let additional_info = buf[offset..].to_vec();

                Descriptor::EnhancedAc3 {
                    mixinfoexists,
                    component_type,
                    bsid,
                    mainid,
                    asvc,
                    substream1,
                    substream2,
                    substream3,
                    additional_info,
                }
            }
            0x7F => {
                //let tag_extension = buf[0];

                let selector_bytes = buf[1..].to_vec();

                Descriptor::Extension { selector_bytes }
            }
            // According to docs, this is "user-defined"... Where are LCN descriptors "officially" defined ???
            0x83 => {
                let mut elements = Vec::new();

                let mut offset = 0;

                while offset < length as usize {
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

                Descriptor::LogicalChannel { elements }
            }
            _ => Descriptor::_Unknown {
                descriptor_id,
                raw_data: buf.to_vec(),
            },
        }
    }

    pub fn read_many(buf: &[u8]) -> Vec<Descriptor> {
        let mut descriptors = Vec::new();

        let mut offset = 0;
        while offset < buf.len() {
            let descriptor_id = buf[offset];
            let length = buf[offset + 1];
            offset += 2;

            let data = &buf[offset..offset + length as usize];
            descriptors.push(Descriptor::read(descriptor_id, length, data));
            offset += length as usize;
        }

        descriptors
    }

    pub fn descriptor_id(&self) -> u8 {
        match self {
            Descriptor::Iso639Language { language: _ } => 0x0A,
            Descriptor::CarouselIdentifier {
                carousel_id: _,
                identifier: _,
            } => 0x13,
            Descriptor::NetworkName { name: _ } => 0x40,
            Descriptor::ServiceList { services: _ } => 0x41,
            Descriptor::Service {
                service_type: _,
                provider: _,
                service: _,
            } => 0x48,
            Descriptor::Component {
                stream_content_ext: _,
                stream_content: _,
                component_type: _,
                component_tag: _,
                language_code: _,
                chars: _,
            } => 0x50,
            Descriptor::StreamIdentifier { component_tag: _ } => 0x52,
            Descriptor::Subtitling { elements: _ } => 0x59,
            Descriptor::TerrestrialDeliverySystem {
                center_frequency: _,
                bandwidth: _,
                priority: _,
                time_slicing_indicator: _,
                mpe_fec_indicator: _,
                constellation: _,
                hierarchy_information: _,
                code_rate_hp_stream: _,
                code_rate_lp_stream: _,
                guard_interval: _,
                transmission_mode: _,
                other_frequency_flag: _,
            } => 0x5A,
            Descriptor::PrivateDataSpecifier { specifier: _ } => 0x5F,
            Descriptor::DataBroadcastId {
                data_broadcast_id: _,
                selector_bytes: _,
            } => 0x66,
            Descriptor::Ac3 {
                component_type: _,
                bsid: _,
                mainid: _,
                asvc: _,
                additional_info_byte: _,
            } => 0x6A,
            Descriptor::ApplicationSignalling { elements: _ } => 0x6F,
            Descriptor::EnhancedAc3 {
                mixinfoexists: _,
                component_type: _,
                bsid: _,
                mainid: _,
                asvc: _,
                substream1: _,
                substream2: _,
                substream3: _,
                additional_info: _,
            } => 0x7A,
            Descriptor::Extension { selector_bytes: _ } => 0x7F,
            Descriptor::LogicalChannel { elements: _ } => 0x83,
            Descriptor::_Unknown {
                descriptor_id: _,
                raw_data: _,
            } => todo!(),
        }
    }
}

fn decode_stupid_string(raw_text: &[u8]) -> Option<String> {
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

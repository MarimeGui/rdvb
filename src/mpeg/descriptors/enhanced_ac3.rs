pub const DESCRIPTOR_ID: u8 = 0x7A;

// ETSI EN 300 468 page 156
#[derive(Debug, Clone)]
pub struct EnhancedAc3 {
    pub mixinfoexists: bool,
    pub component_type: Option<EnhancedAc3ComponentType>,
    pub bsid: Option<u8>,
    pub mainid: Option<u8>,
    pub asvc: Option<u8>,
    pub substream1: Option<u8>,
    pub substream2: Option<u8>,
    pub substream3: Option<u8>,
    pub additional_info: Vec<u8>,
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

impl EnhancedAc3 {
    pub fn from_buf(buf: &[u8]) -> EnhancedAc3 {
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

        EnhancedAc3 {
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
}

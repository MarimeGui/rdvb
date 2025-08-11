//! VDR-style configuration import/export

use std::{num::ParseIntError, str::FromStr};

use crate::error::VdrParseError;

//
// -----

/// A single line of a VDR-style configuration file.
///
/// Example taken from [`man`](https://manpages.ubuntu.com/manpages/xenial/man5/vdr.5.html):
/// ```RTL Television,RTL;RTL World:12187:hC34M2O0S0:S19.2E:27500:163=2:104=deu;106=deu:105:0:12003:1:1089:0```
#[derive(Debug, Clone)]
pub struct ChannelDefinition {
    pub name: String,
    // pub short_name: String,
    pub bouquet: String,
    pub frequency: u32,
    pub parameters: Parameters,
    // Always 'T' for DVB-T and T2
    pub source: String,
    pub symbol_rate: u32,
    pub video_pid: VideoPID,
    pub audio_pid: AudioPIDList,
    pub teletext_pid: TeletextPIDList,
    // `0` for free-to-air
    pub conditional_access: String,
    // program_number in PMT, found in NIT
    pub service_id: u16,
    // Found in NIT
    pub network_id: u16,
    // Found in NIT
    pub transport_stream_id: u16,
    // `0` for television
    pub radio_id: u16,
}

//
// -----

#[derive(Debug, Clone, Default)]
pub struct Parameters {
    pub bandwidth: Option<Bandwidth>,
    pub code_rate_high_priority: Option<CodeRate>,
    pub code_rate_low_priority: Option<CodeRate>,
    pub guard_interval: Option<GuardInterval>,
    pub polarization: Option<Polarization>,
    pub inversion: Option<bool>,
    pub modulation: Option<Modulation>,
    pub pilot_mode: Option<PilotMode>,
    pub roll_off: Option<RollOff>,
    pub stream_id: Option<u8>,
    pub t2_system_id: Option<u16>,
    pub delivery_system_generation: Option<DeliverySystemGeneration>,
    pub transmission_mode: Option<TransmissionMode>,
    pub input_mode: Option<SingleMultipleInput>,
    pub hierarchy: Option<Hierarchy>,
}

#[derive(Debug, Copy, Clone)]
pub enum Bandwidth {
    _1712kHz,
    _5MHz,
    _6Mhz,
    _7MHz,
    _8MHz,
    _10MHz,
}

#[derive(Debug, Copy, Clone)]
pub enum CodeRate {
    NoHierarchy,
    _1_2,
    _2_3,
    _3_4,
    _3_5,
    _4_5,
    _5_6,
    _6_7,
    _7_8,
    _8_9,
    _9_10,
}

#[derive(Debug, Copy, Clone)]
pub enum GuardInterval {
    _1_4,
    _1_8,
    _1_16,
    _1_32,
    _1_128,
    _19_128,
    _19_256,
}

#[derive(Debug, Copy, Clone)]
pub enum Polarization {
    Horizontal,
    Vertical,
    CircularRight,
    CircularLeft,
}

#[derive(Debug, Copy, Clone)]
pub enum Modulation {
    Qpsk,
    _8Psk,
    _16Apsk,
    _32Apsk,
    Vsb8,
    Vsb16,
    Dqpsk,
    Qam16,
    Qam32,
    Qam64,
    Qam128,
    Qam256,
    Auto,
}

#[derive(Debug, Copy, Clone)]
pub enum PilotMode {
    Off,
    On,
    Auto,
}

#[derive(Debug, Copy, Clone)]
pub enum RollOff {
    None,
    _0_20,
    _0_25,
    _0_35,
}

#[derive(Debug, Copy, Clone)]
pub enum DeliverySystemGeneration {
    /// DVB-T, DVB-S2
    FirstGeneration,
    /// DVB-T2, DVB-S2
    SecondGeneration,
}

#[derive(Debug, Copy, Clone)]
pub enum TransmissionMode {
    _1k,
    _2k,
    _4k,
    _8k,
    _16k,
    _32k,
}

#[derive(Debug, Copy, Clone)]
pub enum SingleMultipleInput {
    /// Single-Input Single-Output (SISO)
    SingleInput,
    /// Multiple-Input Single-Output (MISO)
    MultipleInput,
}

#[derive(Debug, Copy, Clone)]
pub enum Hierarchy {
    Off,
    TwoStreams,
    _2,
    _4,
}

impl Parameters {
    // TODO: Could make that into an Iter if I really wanted to
    fn group_params(s: &str) -> Vec<(char, String)> {
        let mut groups = Vec::new();

        let s = s.to_uppercase();
        let mut iter = s.chars();

        let mut letter = if let Some(c) = iter.next() {
            c
        } else {
            return groups;
        };
        let mut data = String::new();

        for c in iter {
            if c.is_ascii_uppercase() {
                // Push old letter and data to groups
                groups.push((letter, data.clone())); // TODO: This could be a permute or something
                data.clear();
                // Change letter
                letter = c;
            } else {
                data.push(c);
            }
        }

        groups.push((letter, data.clone())); // Push last letter and data

        groups
    }
}

impl FromStr for Parameters {
    type Err = VdrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut params = Parameters::default();

        for (letter, data) in Self::group_params(s) {
            // TODO: Move all of these to FromStr on structs ?
            match letter {
                'B' => {
                    params.bandwidth = Some(match data.as_str() {
                        "1712" => Bandwidth::_1712kHz,
                        "5" => Bandwidth::_5MHz,
                        "6" => Bandwidth::_6Mhz,
                        "7" => Bandwidth::_7MHz,
                        "8" => Bandwidth::_8MHz,
                        "10" => Bandwidth::_10MHz,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'C' => {
                    params.code_rate_high_priority = Some(match data.as_str() {
                        "0" => CodeRate::NoHierarchy,
                        "12" => CodeRate::_1_2,
                        "23" => CodeRate::_2_3,
                        "34" => CodeRate::_3_4,
                        "45" => CodeRate::_4_5,
                        "56" => CodeRate::_5_6,
                        "67" => CodeRate::_6_7,
                        "78" => CodeRate::_7_8,
                        "89" => CodeRate::_8_9,
                        "910" => CodeRate::_9_10,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'D' => {
                    params.code_rate_low_priority = Some(match data.as_str() {
                        "0" => CodeRate::NoHierarchy,
                        "12" => CodeRate::_1_2,
                        "23" => CodeRate::_2_3,
                        "34" => CodeRate::_3_4,
                        "45" => CodeRate::_4_5,
                        "56" => CodeRate::_5_6,
                        "67" => CodeRate::_6_7,
                        "78" => CodeRate::_7_8,
                        "89" => CodeRate::_8_9,
                        "910" => CodeRate::_9_10,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'G' => {
                    params.guard_interval = Some(match data.as_str() {
                        "4" => GuardInterval::_1_4,
                        "8" => GuardInterval::_1_8,
                        "16" => GuardInterval::_1_16,
                        "32" => GuardInterval::_1_32,
                        "128" => GuardInterval::_19_128,
                        "19128" => GuardInterval::_19_128,
                        "19256" => GuardInterval::_19_256,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'H' => params.polarization = Some(Polarization::Horizontal),
                'I' => {
                    params.inversion = Some(match data.as_str() {
                        "0" => false,
                        "1" => true,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'L' => params.polarization = Some(Polarization::CircularLeft),
                'M' => {
                    params.modulation = Some(match data.as_str() {
                        "2" => Modulation::Qpsk,
                        "5" => Modulation::_8Psk,
                        "6" => Modulation::_16Apsk,
                        "7" => Modulation::_32Apsk,
                        "10" => Modulation::Vsb8,
                        "11" => Modulation::Vsb16,
                        "12" => Modulation::Dqpsk,
                        "16" => Modulation::Qam16,
                        "32" => Modulation::Qam32,
                        "64" => Modulation::Qam64,
                        "128" => Modulation::Qam128,
                        "256" => Modulation::Qam256,
                        "999" => Modulation::Auto,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'N' => {
                    params.pilot_mode = Some(match data.as_str() {
                        "0" => PilotMode::Off,
                        "1" => PilotMode::On,
                        "999" => PilotMode::Auto,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'O' => {
                    params.roll_off = Some(match data.as_str() {
                        "0" => RollOff::None,
                        "20" => RollOff::_0_20,
                        "25" => RollOff::_0_25,
                        "35" => RollOff::_0_35,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'P' => params.stream_id = Some(data.parse().map_err(VdrParseError::IntParse)?),
                'Q' => params.t2_system_id = Some(data.parse().map_err(VdrParseError::IntParse)?),
                'R' => params.polarization = Some(Polarization::CircularRight),
                'S' => {
                    params.delivery_system_generation = Some(match data.as_str() {
                        "0" => DeliverySystemGeneration::FirstGeneration,
                        "1" => DeliverySystemGeneration::SecondGeneration,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'T' => {
                    params.transmission_mode = Some(match data.as_str() {
                        "1" => TransmissionMode::_1k,
                        "2" => TransmissionMode::_2k,
                        "4" => TransmissionMode::_4k,
                        "8" => TransmissionMode::_8k,
                        "16" => TransmissionMode::_16k,
                        "32" => TransmissionMode::_32k,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'V' => params.polarization = Some(Polarization::Vertical),
                'X' => {
                    params.input_mode = Some(match data.as_str() {
                        "0" => SingleMultipleInput::SingleInput,
                        "1" => SingleMultipleInput::MultipleInput,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                'Y' => {
                    params.hierarchy = Some(match data.as_str() {
                        "0" => Hierarchy::Off,
                        "1" => Hierarchy::TwoStreams,
                        "2" => Hierarchy::_2,
                        "4" => Hierarchy::_4,
                        _ => return Err(VdrParseError::UnexpectedParameterValue),
                    })
                }
                _ => return Err(VdrParseError::UnknownParameter),
            }
        }

        Ok(params)
    }
}

//
// -----

#[derive(Debug, Clone)]
pub struct VideoPID {
    pub pcr_pid: u16,
    pub video_pid: Option<u16>,
    pub video_mode: Option<u16>,
}

impl FromStr for VideoPID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once('+') {
            Some((vpid, rest)) => {
                match rest.split_once('=') {
                    // Separated Video PID, PCR PID and Video mode (like "164+17=27")
                    Some((pcr_pid, video_mode)) => VideoPID {
                        pcr_pid: pcr_pid.parse()?,
                        video_pid: Some(vpid.parse()?),
                        video_mode: Some(video_mode.parse()?),
                    },
                    // Separate Video PID and PCR PID (like "164+17")
                    None => VideoPID {
                        pcr_pid: vpid.parse()?,
                        video_pid: Some(rest.parse()?),
                        video_mode: None,
                    },
                }
            }
            None => {
                match s.split_once('=') {
                    // Separated PCR PID and Video mode (like "164=27")
                    Some((pcr_pid, video_mode)) => VideoPID {
                        pcr_pid: pcr_pid.parse()?,
                        video_pid: None,
                        video_mode: Some(video_mode.parse()?),
                    },
                    // Only PCR PID (like "164")
                    None => VideoPID {
                        pcr_pid: s.parse()?,
                        video_pid: None,
                        video_mode: None,
                    },
                }
            }
        })
    }
}

//
// -----

#[derive(Debug, Clone)]
pub struct AudioPIDList {
    pub regular_pids: Vec<AudioPID>,
    pub dolby_pids: Vec<AudioPID>,
}

#[derive(Debug, Clone)]
pub struct AudioPID {
    pub pid: u16,
    pub language_code: String,
    pub second_language_code: String,
    pub audio_type: Option<u16>,
}

impl AudioPIDList {
    fn parse_part(s: &str) -> Result<Vec<AudioPID>, ParseIntError> {
        let mut entries = Vec::new();
        for one_pid in s.split(',') {
            entries.push(one_pid.parse()?);
        }
        Ok(entries)
    }
}

impl FromStr for AudioPIDList {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(';') {
            Some((regular, dolby)) => {
                if regular == "0" {
                    AudioPIDList {
                        regular_pids: vec![],
                        dolby_pids: Self::parse_part(dolby)?,
                    }
                } else {
                    AudioPIDList {
                        regular_pids: Self::parse_part(regular)?,
                        dolby_pids: Self::parse_part(dolby)?,
                    }
                }
            }
            None => AudioPIDList {
                regular_pids: Self::parse_part(s)?,
                dolby_pids: vec![],
            },
        })
    }
}

impl FromStr for AudioPID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try splitting off the audio type first
        let (rest, audio_type) = match s.split_once('@') {
            Some((rest, audio_type)) => (rest, Some(audio_type.parse()?)),
            None => (s, None),
        };

        // Try splitting the language
        let (pid, languages) = match rest.split_once('=') {
            Some((pid, langauges)) => (pid.parse()?, Some(langauges)),
            None => (rest.parse()?, None),
        };

        // Finally, if there is a language string, try splitting it
        let (language_code, second_language_code) = match languages {
            Some(l) => match l.split_once('+') {
                Some((first, second)) => (first.to_string(), second.to_string()),
                None => (l.to_string(), String::new()),
            },
            None => (String::new(), String::new()),
        };

        Ok(AudioPID {
            pid,
            language_code,
            second_language_code,
            audio_type,
        })
    }
}

//
// -----

#[derive(Debug, Clone, Default)]
pub struct TeletextPIDList {
    pub teletext: Vec<u16>,
    pub subtitles: Vec<SubtitlePID>,
}

#[derive(Debug, Clone)]
pub struct SubtitlePID {
    pub pid: u16,
    pub language: String,
}

impl TeletextPIDList {
    fn parse_teletext(s: &str) -> Result<Vec<u16>, ParseIntError> {
        let mut teletext = Vec::new();
        for p in s.split(',') {
            teletext.push(p.parse()?);
        }
        Ok(teletext)
    }

    fn parse_subtitles(s: &str) -> Result<Vec<SubtitlePID>, ParseIntError> {
        let mut subtitles = Vec::new();
        for p in s.split(',') {
            subtitles.push(p.parse()?);
        }
        Ok(subtitles)
    }
}

impl FromStr for TeletextPIDList {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "0" {
            return Ok(TeletextPIDList::default());
        }

        Ok(match s.split_once(';') {
            Some((teletext, subtitles)) => {
                if teletext == "0" {
                    // Just subtitles
                    TeletextPIDList {
                        teletext: vec![],
                        subtitles: Self::parse_subtitles(subtitles)?,
                    }
                } else {
                    // Both teletext and subtitles
                    TeletextPIDList {
                        teletext: Self::parse_teletext(teletext)?,
                        subtitles: Self::parse_subtitles(subtitles)?,
                    }
                }
            }
            None => {
                // Just teletext
                TeletextPIDList {
                    teletext: Self::parse_teletext(s)?,
                    subtitles: vec![],
                }
            }
        })
    }
}

impl FromStr for SubtitlePID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once('=') {
            Some((pid, lang)) => SubtitlePID {
                pid: pid.parse()?,
                language: lang.to_string(),
            },
            None => SubtitlePID {
                pid: s.parse()?,
                language: String::new(),
            },
        })
    }
}

//
// -----

impl FromStr for ChannelDefinition {
    type Err = VdrParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut iter = line.split(':');

        let name_bouquet = iter.next().ok_or(VdrParseError::MissingColumn)?;
        let frequency = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let parameters = iter.next().ok_or(VdrParseError::MissingColumn)?.parse()?;
        let source = iter.next().ok_or(VdrParseError::MissingColumn)?.to_string();
        let symbol_rate = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .to_string()
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let video_pid = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let audio_pid = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let teletext_pid = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let conditional_access = iter.next().ok_or(VdrParseError::MissingColumn)?.to_string();
        let service_id = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let network_id = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let transport_stream_id = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let radio_id = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;

        // Separate name and bouquet
        let (name, bouquet) = if let Some((n, b)) = name_bouquet.rsplit_once(';') {
            (n.replace("|", ":"), b.to_string())
        } else {
            (name_bouquet.to_string(), String::new())
        };

        Ok(ChannelDefinition {
            name,
            bouquet,
            frequency,
            parameters,
            source,
            symbol_rate,
            video_pid,
            audio_pid,
            teletext_pid,
            conditional_access,
            service_id,
            network_id,
            transport_stream_id,
            radio_id,
        })
    }
}

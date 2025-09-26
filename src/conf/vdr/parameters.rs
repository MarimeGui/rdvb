use std::str::FromStr;

use crate::{
    error::VdrParseError,
    frontend::{DeliverySystemGeneration, properties::set::BandwidthHz},
};

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

// TODO: Could generalize
#[derive(Debug, Copy, Clone)]
pub enum Bandwidth {
    _1712kHz,
    _5MHz,
    _6Mhz,
    _7MHz,
    _8MHz,
    _10MHz,
}

impl From<BandwidthHz> for Bandwidth {
    fn from(value: BandwidthHz) -> Self {
        match value {
            BandwidthHz::_1_172MHz => Bandwidth::_1712kHz,
            BandwidthHz::_5MHz => Bandwidth::_5MHz,
            BandwidthHz::_6MHz => Bandwidth::_6Mhz,
            BandwidthHz::_7MHz => Bandwidth::_7MHz,
            BandwidthHz::_8MHz => Bandwidth::_8MHz,
            BandwidthHz::_10MHz => Bandwidth::_10MHz,
        }
    }
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

impl Bandwidth {
    pub fn format(self) -> &'static str {
        match self {
            Bandwidth::_1712kHz => "B1712",
            Bandwidth::_5MHz => "B5",
            Bandwidth::_6Mhz => "B6",
            Bandwidth::_7MHz => "B7",
            Bandwidth::_8MHz => "B8",
            Bandwidth::_10MHz => "B10",
        }
    }
}

impl CodeRate {
    pub fn partial_format(self) -> &'static str {
        match self {
            CodeRate::NoHierarchy => "0",
            CodeRate::_1_2 => "12",
            CodeRate::_2_3 => "23",
            CodeRate::_3_4 => "34",
            CodeRate::_3_5 => "35",
            CodeRate::_4_5 => "45",
            CodeRate::_5_6 => "56",
            CodeRate::_6_7 => "67",
            CodeRate::_7_8 => "78",
            CodeRate::_8_9 => "89",
            CodeRate::_9_10 => "910",
        }
    }
}

impl GuardInterval {
    pub fn format(self) -> &'static str {
        match self {
            GuardInterval::_1_4 => "G4",
            GuardInterval::_1_8 => "G8",
            GuardInterval::_1_16 => "G16",
            GuardInterval::_1_32 => "G32",
            GuardInterval::_1_128 => "G128",
            GuardInterval::_19_128 => "G19128",
            GuardInterval::_19_256 => "G19256",
        }
    }
}

impl Polarization {
    pub fn format(self) -> char {
        match self {
            Polarization::Horizontal => 'H',
            Polarization::CircularLeft => 'L',
            Polarization::CircularRight => 'R',
            Polarization::Vertical => 'V',
        }
    }
}

impl Modulation {
    pub fn format(self) -> &'static str {
        match self {
            Modulation::Qpsk => "M2",
            Modulation::_8Psk => "M5",
            Modulation::_16Apsk => "M6",
            Modulation::_32Apsk => "M7",
            Modulation::Vsb8 => "M10",
            Modulation::Vsb16 => "M11",
            Modulation::Dqpsk => "M12",
            Modulation::Qam16 => "M16",
            Modulation::Qam32 => "M32",
            Modulation::Qam64 => "M64",
            Modulation::Qam128 => "M128",
            Modulation::Qam256 => "M256",
            Modulation::Auto => "M999",
        }
    }
}

impl PilotMode {
    pub fn format(self) -> &'static str {
        match self {
            PilotMode::Off => "N0",
            PilotMode::On => "N1",
            PilotMode::Auto => "N999",
        }
    }
}

impl RollOff {
    pub fn format(self) -> &'static str {
        match self {
            RollOff::None => "O0",
            RollOff::_0_20 => "O20",
            RollOff::_0_25 => "O25",
            RollOff::_0_35 => "O35",
        }
    }
}

impl DeliverySystemGeneration {
    pub fn format(self) -> &'static str {
        match self {
            DeliverySystemGeneration::FirstGeneration => "S0",
            DeliverySystemGeneration::SecondGeneration => "S1",
        }
    }
}

impl TransmissionMode {
    pub fn format(self) -> &'static str {
        match self {
            TransmissionMode::_1k => "T1",
            TransmissionMode::_2k => "T2",
            TransmissionMode::_4k => "T4",
            TransmissionMode::_8k => "T8",
            TransmissionMode::_16k => "T16",
            TransmissionMode::_32k => "T32",
        }
    }
}

impl SingleMultipleInput {
    pub fn format(self) -> &'static str {
        match self {
            SingleMultipleInput::SingleInput => "X0",
            SingleMultipleInput::MultipleInput => "X1",
        }
    }
}

impl Hierarchy {
    pub fn format(self) -> &'static str {
        match self {
            Hierarchy::Off => "Y0",
            Hierarchy::TwoStreams => "Y1",
            Hierarchy::_2 => "Y2",
            Hierarchy::_4 => "Y4",
        }
    }
}

impl Parameters {
    pub fn format(&self) -> String {
        let mut text = String::new();

        if let Some(bandwidth) = self.bandwidth {
            text.push_str(bandwidth.format());
        }

        if let Some(code_rate) = self.code_rate_high_priority {
            text.push('C');
            text.push_str(code_rate.partial_format());
        }

        if let Some(code_rate) = self.code_rate_low_priority {
            text.push('D');
            text.push_str(code_rate.partial_format());
        }

        if let Some(guard_interval) = self.guard_interval {
            text.push_str(guard_interval.format());
        }

        if let Some(polarization) = self.polarization {
            text.push(polarization.format());
        }

        if let Some(inversion) = self.inversion {
            if inversion {
                text.push_str("I1")
            } else {
                text.push_str("I0")
            }
        }

        if let Some(modulation) = self.modulation {
            text.push_str(modulation.format());
        }

        if let Some(pilot_mode) = self.pilot_mode {
            text.push_str(pilot_mode.format());
        }

        if let Some(roll_off) = self.roll_off {
            text.push_str(roll_off.format());
        }

        if let Some(id) = self.stream_id {
            text.push_str(&format!("P{}", id));
        }

        if let Some(id) = self.t2_system_id {
            text.push_str(&format!("Q{}", id));
        }

        if let Some(generation) = self.delivery_system_generation {
            text.push_str(generation.format());
        }

        if let Some(mode) = self.transmission_mode {
            text.push_str(mode.format());
        }

        if let Some(siso_miso) = self.input_mode {
            text.push_str(siso_miso.format());
        }

        if let Some(hierarchy) = self.hierarchy {
            text.push_str(hierarchy.format());
        }

        text
    }
}

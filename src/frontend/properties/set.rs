use crate::frontend::sys::{
    FeCodeRate, FeDeliverySystem, FeGuardInterval, FeModulation, FeSpectralInversion,
    FeTransmitMode,
    property::{Command, DtvProperty},
};

//
// ----- Common trait
pub trait SetPropertyQuery {
    fn property(self) -> DtvProperty;
}

//
// ----- Individual queries

// TODO: Macro for all "simple" single data properties

pub struct Tune {}
impl SetPropertyQuery for Tune {
    fn property(self) -> DtvProperty {
        DtvProperty::new_empty(Command::DTV_TUNE)
    }
}

// --

pub struct Clear {}
impl SetPropertyQuery for Clear {
    fn property(self) -> DtvProperty {
        DtvProperty::new_empty(Command::DTV_CLEAR)
    }
}

// --

pub struct Frequency(u32);
impl Frequency {
    pub fn new(frequency: u32) -> Frequency {
        Frequency(frequency)
    }
}
impl SetPropertyQuery for Frequency {
    fn property(self) -> DtvProperty {
        DtvProperty::new_data(Command::DTV_FREQUENCY, self.0)
    }
}

// --

pub struct Modulation(FeModulation);
impl Modulation {
    pub fn new(modulation: FeModulation) -> Modulation {
        Modulation(modulation)
    }
}
impl SetPropertyQuery for Modulation {
    fn property(self) -> DtvProperty {
        DtvProperty::new_data(Command::DTV_MODULATION, self.0 as u32)
    }
}

// --

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BandwidthHz {
    _1_172MHz,
    _5MHz,
    _6MHz,
    _7MHz,
    _8MHz,
    _10MHz,
}
impl BandwidthHz {
    pub fn value(&self) -> u32 {
        match self {
            BandwidthHz::_1_172MHz => 1712000,
            BandwidthHz::_5MHz => 5000000,
            BandwidthHz::_6MHz => 6000000,
            BandwidthHz::_7MHz => 7000000,
            BandwidthHz::_8MHz => 8000000,
            BandwidthHz::_10MHz => 10000000,
        }
    }
}
impl SetPropertyQuery for BandwidthHz {
    fn property(self) -> DtvProperty {
        DtvProperty::new_data(Command::DTV_BANDWIDTH_HZ, self.value())
    }
}

// --

pub struct Inversion(FeSpectralInversion);
impl Inversion {
    pub fn new(inversion: FeSpectralInversion) -> Inversion {
        Inversion(inversion)
    }
}
impl SetPropertyQuery for Inversion {
    fn property(self) -> DtvProperty {
        DtvProperty::new_data(Command::DTV_INVERSION, self.0 as u32)
    }
}

// --

pub struct SymbolRate {}

// --

pub struct InnerFec(FeCodeRate);
impl InnerFec {
    pub fn new(rate: FeCodeRate) -> InnerFec {
        InnerFec(rate)
    }
}
impl SetPropertyQuery for InnerFec {
    fn property(self) -> DtvProperty {
        DtvProperty::new_data(Command::DTV_INNER_FEC, self.0 as u32)
    }
}

// --

pub struct Pilot {}

// --

pub struct Rolloff {}

// --

pub struct DeliverySystem(FeDeliverySystem);
impl DeliverySystem {
    pub fn new(system: FeDeliverySystem) -> DeliverySystem {
        DeliverySystem(system)
    }
}
impl SetPropertyQuery for DeliverySystem {
    fn property(self) -> DtvProperty {
        DtvProperty::new_data(Command::DTV_DELIVERY_SYSTEM, self.0 as u32)
    }
}

// --

// Special
pub struct Voltage {}

// --

// Special
pub struct Tone {}

// --

pub struct CodeRateHp(FeTransmitMode);
impl CodeRateHp {
    pub fn new(mode: FeTransmitMode) -> CodeRateHp {
        CodeRateHp(mode)
    }
}
impl SetPropertyQuery for CodeRateHp {
    fn property(self) -> DtvProperty {
        DtvProperty::new_data(Command::DTV_CODE_RATE_HP, self.0 as u32)
    }
}

// --

pub struct CodeRateLp(FeTransmitMode);
impl CodeRateLp {
    pub fn new(mode: FeTransmitMode) -> CodeRateLp {
        CodeRateLp(mode)
    }
}
impl SetPropertyQuery for CodeRateLp {
    fn property(self) -> DtvProperty {
        DtvProperty::new_data(Command::DTV_CODE_RATE_LP, self.0 as u32)
    }
}

// --

pub struct GuardInterval(FeGuardInterval);
impl GuardInterval {
    pub fn new(interval: FeGuardInterval) -> GuardInterval {
        GuardInterval(interval)
    }
}
impl SetPropertyQuery for GuardInterval {
    fn property(self) -> DtvProperty {
        DtvProperty::new_data(Command::DTV_GUARD_INTERVAL, self.0 as u32)
    }
}

// --

pub struct TransmissionMode {}

// --

pub struct Hierarchy {}

// --

pub struct Interleaving {}

// TODO: ISDB-T, Multistream, Physical layer scrambling, ATSC-MH

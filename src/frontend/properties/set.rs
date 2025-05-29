use crate::frontend::sys::{
    FeDeliverySystem, FeModulation, FeSpectralInversion,
    property::{Command, DtvProperty},
};

//
// ----- Common trait
pub trait SetPropertyQuery {
    fn property(self) -> DtvProperty;
}

//
// ----- Individual queries

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

pub enum Bandwidth {
    _1_172MHz,
    _5MHz,
    _6MHz,
    _7MHz,
    _8MHz,
    _10MHz,
}
impl Bandwidth {
    pub fn value(&self) -> u32 {
        match self {
            Bandwidth::_1_172MHz => 1712000,
            Bandwidth::_5MHz => 5000000,
            Bandwidth::_6MHz => 6000000,
            Bandwidth::_7MHz => 7000000,
            Bandwidth::_8MHz => 8000000,
            Bandwidth::_10MHz => 10000000,
        }
    }
}
impl SetPropertyQuery for Bandwidth {
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

pub struct InnerFec {}

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

pub struct CodeRateHp {}

// --

pub struct CodeRateLp {}

// --

pub struct GuardInterval {}

// --

pub struct TransmissionMode {}

// --

pub struct Hierarchy {}

// --

pub struct Interleaving {}

// TODO: ISDB-T, Multistream, Physical layer scrambling, ATSC-MH

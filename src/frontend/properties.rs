use std::marker::PhantomData;

use super::sys::{DtvProperty, DtvPropertyUnion, FeDeliverySystem, PropertyCommands};

//
// -----

pub trait PropertyQuery {
    fn associated_command() -> PropertyCommands;
    fn from_property(u: DtvPropertyUnion) -> Self;

    /// Create a PendingQuery that can be passed to the properties method of a Frontend.
    ///
    /// After properties() has run, use retrieve() to get the actual value back.
    fn new() -> PendingQuery<Self>
    where
        Self: Sized,
    {
        PendingQuery {
            phantom: PhantomData,
            memory: None,
        }
    }
}

#[derive(Default)]
pub struct PendingQuery<T> {
    phantom: PhantomData<T>,
    memory: Option<DtvProperty>,
}

impl<T: PropertyQuery> PendingQuery<T> {
    pub fn retrieve(self) -> Option<T> {
        let property = self.memory?;
        if property.result < 0 {
            return None;
        }
        Some(T::from_property(property.u))
    }

    pub fn desc(&mut self) -> (PropertyCommands, &mut Option<DtvProperty>) {
        (T::associated_command(), &mut self.memory)
    }
}

//
// -----

/// Use to figure out what transmission systems (DVB-S, DVB-T...) the frontend can work with.
///
/// From [https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/fe_property_parameters.html#dtv-enum-delsys]:
///
/// "A Multi standard frontend needs to advertise the delivery systems provided.
/// Applications need to enumerate the provided delivery systems,
/// before using any other operation with the frontend.
/// Prior to it’s introduction,
/// FE_GET_INFO was used to determine a frontend type.
/// A frontend which provides more than a single delivery system,
/// FE_GET_INFO doesn’t help much.
/// Applications which intends to use a multistandard frontend must
/// enumerate the delivery systems associated with it,
/// rather than trying to use FE_GET_INFO.
/// In the case of a legacy frontend,
/// the result is just the same as with FE_GET_INFO,
/// but in a more structured format"
#[derive(Debug)]
pub struct EnumerateDeliverySystems(pub FeDeliverySystem);
impl PropertyQuery for EnumerateDeliverySystems {
    fn associated_command() -> PropertyCommands {
        PropertyCommands::EnumDelSys
    }

    fn from_property(u: DtvPropertyUnion) -> Self {
        Self(FeDeliverySystem(unsafe { u.data }))
    }
}

// ---

#[derive(Debug)]
pub struct Frequency(pub u32);
impl PropertyQuery for Frequency {
    fn associated_command() -> PropertyCommands {
        PropertyCommands::Frequency
    }

    fn from_property(u: DtvPropertyUnion) -> Self {
        Self(unsafe { u.data })
    }
}

// TODO: Return correct UOM when given system ?

// ---

#[derive(Debug)]
pub enum SignalStrength {
    None,
    Decibel(i64),
    Relative(u64),
}
impl PropertyQuery for SignalStrength {
    fn associated_command() -> PropertyCommands {
        PropertyCommands::StatSignalStrength
    }

    fn from_property(u: DtvPropertyUnion) -> Self {
        let stats = unsafe { u.st };
        assert_eq!(stats.len, 1);
        let stat = stats.stat[0];
        match stat.scale {
            1 => SignalStrength::Decibel(unsafe { stat.__bindgen_anon_1.svalue }),
            2 => SignalStrength::Relative(unsafe { stat.__bindgen_anon_1.uvalue }),
            _ => SignalStrength::None,
        }
    }
}

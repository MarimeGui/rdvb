use std::marker::PhantomData;

use crate::frontend::sys::{
    FeDeliverySystem,
    property::{Command, DtvProperty, DtvPropertyUnion},
};

//
// ----- Common trait and structs

pub trait PropertyQuery {
    fn associated_command() -> Command;
    fn from_property(u: DtvPropertyUnion) -> Self;

    /// Create a PendingQuery that can be passed to the properties method of a Frontend.
    ///
    /// After properties() has run, use retrieve() to get the actual value back.
    fn query() -> PendingQuery<Self>
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

pub struct QueryDescription<'a> {
    pub(crate) command: Command,
    pub(crate) property: &'a mut Option<DtvProperty>,
}

impl<T: PropertyQuery> PendingQuery<T> {
    pub fn retrieve(self) -> Option<T> {
        let property = self.memory?;
        if property.result < 0 {
            return None;
        }
        Some(T::from_property(property.u))
    }

    pub fn desc(&mut self) -> QueryDescription {
        QueryDescription {
            command: T::associated_command(),
            property: &mut self.memory,
        }
    }
}

//
// ----- Individual queries

#[derive(Debug)]
pub struct EnumerateDeliverySystems(pub Vec<FeDeliverySystem>);
impl PropertyQuery for EnumerateDeliverySystems {
    fn associated_command() -> Command {
        Command::DTV_ENUM_DELSYS
    }

    fn from_property(u: DtvPropertyUnion) -> Self {
        let len = unsafe { u.buffer.len } as usize;

        let mut systems = Vec::with_capacity(len);
        for i in 0..len {
            let data = unsafe { u.buffer.data[i] };
            systems.push(FeDeliverySystem::try_from(data).unwrap());
        }

        EnumerateDeliverySystems(systems)
    }
}

// ---

#[derive(Debug)]
pub struct Frequency(pub u32);
impl PropertyQuery for Frequency {
    fn associated_command() -> Command {
        Command::DTV_FREQUENCY
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
    fn associated_command() -> Command {
        Command::DTV_STAT_SIGNAL_STRENGTH
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

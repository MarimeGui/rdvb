use std::{collections::HashSet, marker::PhantomData};

use crate::frontend::sys::{
    FeDeliverySystem, FeModulation,
    property::{Command, DtvProperty, DtvPropertyUnion, DtvStatsValue, FeCapScaleParams},
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

pub enum StatResult {
    Value(ValueStat),
    Count(u64),
}

#[derive(Debug)]
pub enum ValueStat {
    Decibel(i64),
    Relative(u64),
}

impl StatResult {
    fn from(scale: FeCapScaleParams, raw_value: DtvStatsValue) -> Option<StatResult> {
        match scale {
            FeCapScaleParams::FE_SCALE_NOT_AVAILABLE => None,
            FeCapScaleParams::FE_SCALE_DECIBEL => {
                Some(StatResult::Value(ValueStat::Decibel(unsafe {
                    raw_value.svalue
                })))
            }
            FeCapScaleParams::FE_SCALE_RELATIVE => {
                Some(StatResult::Value(ValueStat::Relative(unsafe {
                    raw_value.uvalue
                })))
            }
            FeCapScaleParams::FE_SCALE_COUNTER => {
                Some(StatResult::Count(unsafe { raw_value.uvalue }))
            }
        }
    }
}

//
// ----- Individual queries

#[derive(Debug)]
pub struct EnumerateDeliverySystems(pub HashSet<FeDeliverySystem>);
impl PropertyQuery for EnumerateDeliverySystems {
    fn associated_command() -> Command {
        Command::DTV_ENUM_DELSYS
    }

    fn from_property(u: DtvPropertyUnion) -> Self {
        let len = unsafe { u.buffer.len } as usize;

        let mut systems = HashSet::with_capacity(len);
        for i in 0..len {
            let data = unsafe { u.buffer.data[i] };
            systems.insert(FeDeliverySystem::try_from(data).unwrap());
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
pub struct Modulation(pub FeModulation);
impl PropertyQuery for Modulation {
    fn associated_command() -> Command {
        Command::DTV_MODULATION
    }

    fn from_property(u: DtvPropertyUnion) -> Self {
        Self(unsafe {
            FeModulation::try_from(u.data).expect("unexpected value for modulation type")
        })
    }
}

// ---

#[derive(Debug)]
pub struct SignalStrength(pub Option<ValueStat>);
impl PropertyQuery for SignalStrength {
    fn associated_command() -> Command {
        Command::DTV_STAT_SIGNAL_STRENGTH
    }

    fn from_property(u: DtvPropertyUnion) -> Self {
        let stats = unsafe { u.st };
        assert_eq!(stats.len, 1);
        let stat = stats.stat[0];
        let scale = FeCapScaleParams::try_from(stat.scale).expect("unexpected value for stat type");
        let res = match StatResult::from(scale, stat.value) {
            Some(v) => v,
            None => return Self(None),
        };
        match res {
            StatResult::Value(value_stat) => Self(Some(value_stat)),
            StatResult::Count(_) => panic!("expected a value, not a count"),
        }
    }
}

// --

#[derive(Debug)]
pub struct CarrierSignalToNoise(pub Option<ValueStat>);

// --

#[derive(Debug)]
pub struct TotalBlockCount(pub Option<u64>);
impl PropertyQuery for TotalBlockCount {
    fn associated_command() -> Command {
        Command::DTV_STAT_TOTAL_BLOCK_COUNT
    }

    fn from_property(u: DtvPropertyUnion) -> Self {
        let stats = unsafe { u.st };
        assert_eq!(stats.len, 1);
        let stat = stats.stat[0];
        let scale = FeCapScaleParams::try_from(stat.scale).expect("unexpected value for stat type");
        let res = match StatResult::from(scale, stat.value) {
            Some(v) => v,
            None => return Self(None),
        };
        match res {
            StatResult::Value(_) => panic!("expected a count, not a value"),
            StatResult::Count(count) => Self(Some(count)),
        }
    }
}

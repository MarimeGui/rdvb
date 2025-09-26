use std::{collections::BTreeSet, marker::PhantomData};

use rdvb_os_linux::frontend::{
    data::{FeDeliverySystem, FeModulation},
    property::{Command, DtvProperty, DtvPropertyUnion, DtvStatsValue, FeCapScaleParams},
};

use crate::error::DtvError;

// Complete list can be found in linux/drivers/media/dvb-core/dvb_frontend.c -> dtv_property_process_get

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
    pub fn retrieve(self) -> Result<T, DtvError> {
        let property = self.memory.ok_or(DtvError::NotRan)?;
        if property.result < 0 {
            return Err(DtvError::Reported(property.result));
        }
        Ok(T::from_property(property.u))
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ValueStat {
    Decibel(i64),
    Relative(u64),
}

impl StatResult {
    fn from(scale: FeCapScaleParams, raw_value: DtvStatsValue) -> Option<StatResult> {
        match scale {
            FeCapScaleParams::FE_SCALE_NOT_AVAILABLE => None,
            FeCapScaleParams::FE_SCALE_DECIBEL => {
                // SAFETY: This is always safe, as the union all interpretations of the union would yield a valid int.
                Some(StatResult::Value(ValueStat::Decibel(unsafe {
                    raw_value.svalue
                })))
            }
            FeCapScaleParams::FE_SCALE_RELATIVE => {
                // SAFETY: This is always safe, as the union all interpretations of the union would yield a valid int.
                Some(StatResult::Value(ValueStat::Relative(unsafe {
                    raw_value.uvalue
                })))
            }
            FeCapScaleParams::FE_SCALE_COUNTER => {
                // SAFETY: This is always safe, as the union all interpretations of the union would yield a valid int.
                Some(StatResult::Count(unsafe { raw_value.uvalue }))
            }
        }
    }
}

impl PartialOrd for ValueStat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (ValueStat::Decibel(_a), ValueStat::Decibel(_b)) => {
                todo!("no idea how the dB info is encoded")
            }
            (ValueStat::Relative(a), ValueStat::Relative(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

//
// ----- Individual queries

/// List of supported delivery systems by an adapter.
///
/// This is using a BTreeSet for two reasons :
/// - Ensure uniqueness of each system,
/// - When iterating, do older systems first as they're most likely to have the most channels, making newer systems faster to scan.
#[derive(Debug)]
pub struct EnumerateDeliverySystems(pub BTreeSet<FeDeliverySystem>);
impl PropertyQuery for EnumerateDeliverySystems {
    fn associated_command() -> Command {
        Command::DTV_ENUM_DELSYS
    }

    fn from_property(u: DtvPropertyUnion) -> Self {
        let len = unsafe { u.buffer.len } as usize;

        let mut systems = BTreeSet::new();
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

pub struct SymbolRate(pub u32);
impl PropertyQuery for SymbolRate {
    fn associated_command() -> Command {
        Command::DTV_SYMBOL_RATE
    }

    fn from_property(u: DtvPropertyUnion) -> Self {
        // SAFETY: No matter what data is provided, a u32 always has a valid value
        Self(unsafe { u.data })
    }
}

// ---

#[derive(Debug, PartialEq, Eq)]
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

impl PartialOrd for SignalStrength {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.0, other.0) {
            (None, None) => None,
            (None, Some(_)) => Some(std::cmp::Ordering::Less),
            (Some(_), None) => Some(std::cmp::Ordering::Greater),
            (Some(a), Some(b)) => a.partial_cmp(&b),
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

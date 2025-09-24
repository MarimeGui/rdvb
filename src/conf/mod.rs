//! Channel configuration files for use with other programs or with this library

use crate::frontend::{properties::set::BandwidthHz, sys::FeDeliverySystem};

pub mod vdr;

/// A single logical channel, as in an actual TV channel.
///
/// This is available after analysis of the transponder data received from the air.
pub struct ChannelInformation {
    pub bandwidth: BandwidthHz,
    pub delivery_system: FeDeliverySystem,
    pub name: String,
    pub frequency: u32,
    pub logical_channel_number: u16,
    pub service_id: u16,
    pub original_network_id: u16,
    pub transport_stream_id: u16,
}

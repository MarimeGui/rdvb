use crate::mpeg::{
    descriptors::{
        ac3::Ac3, application_signalling::ApplicationSignalling,
        carousel_identifier::CarouselIdentifier, component::Component,
        data_broadcast_id::DataBroadcastId, enhanced_ac3::EnhancedAc3, extension::Extension,
        iso639_language::Iso639Language, logical_channel::LogicalChannel,
        network_name::NetworkName, private_data_specifier::PrivateDataSpecifier, service::Service,
        service_list::ServiceList, stream_identifier::StreamIdentifier, subtitling::Subtitling,
        terrestrial_delivery_system::TerrestrialDeliverySystem,
    },
};

pub mod ac3;
pub mod application_signalling;
pub mod carousel_identifier;
pub mod component;
pub mod data_broadcast_id;
pub mod enhanced_ac3;
pub mod extension;
pub mod iso639_language;
pub mod logical_channel;
pub mod network_name;
pub mod private_data_specifier;
pub mod service;
pub mod service_list;
pub mod stream_identifier;
pub mod subtitling;
pub mod terrestrial_delivery_system;

// -----

// Also look in vdr si.h DescriptorTag enum
#[derive(Debug, Clone)]
pub enum Descriptor {
    NetworkName(NetworkName),
    ServiceList(ServiceList),
    Service(Service),
    StreamIdentifier(StreamIdentifier),
    TerrestrialDeliverySystem(TerrestrialDeliverySystem),
    LogicalChannel(LogicalChannel),
    EnhancedAc3(EnhancedAc3),
    PrivateDataSpecifier(PrivateDataSpecifier),
    DataBroadcastId(DataBroadcastId),
    Extension(Extension),
    Subtitling(Subtitling),
    Component(Component),
    Iso639Language(Iso639Language),
    ApplicationSignalling(ApplicationSignalling),
    Ac3(Ac3),
    CarouselIdentifier(CarouselIdentifier),
    _Unknown(UnknownDescriptor),
}

// #[derive(Debug, Clone)]
// pub struct UnknownDescriptor {
//     /// Same as program number in program map except for 0x04, 0x18, 0x1B (NVOD services) (from ETSI EN 300 468)
//     pub service_id: u16,
//     pub service_type: ServiceType,
// }

#[derive(Debug, Clone)]
pub struct UnknownDescriptor {
    pub descriptor_id: u8,
    pub raw_data: Vec<u8>,
}

impl Descriptor {
    pub fn read(descriptor_id: u8, buf: &[u8]) -> Descriptor {
        // TODO: Could write macro
        match descriptor_id {
            // 0x05 => {} // In TS 102 809, but does not correspond to the data I'm getting
            iso639_language::DESCRIPTOR_ID => {
                Descriptor::Iso639Language(Iso639Language::from_buf(buf))
            }
            // 0x09 => {} // In TS 102 809, but does not correspond to the data I'm getting
            // 0x0E => {}, // Seen on a DVB-T2 HEVC channel
            carousel_identifier::DESCRIPTOR_ID => {
                Descriptor::CarouselIdentifier(CarouselIdentifier::from_buf(buf))
            }
            // 0x38 => {}, // Seen on a DVB-T2 HEVC channel
            network_name::DESCRIPTOR_ID => Descriptor::NetworkName(NetworkName::from_buf(buf)),
            service_list::DESCRIPTOR_ID => Descriptor::ServiceList(ServiceList::from_buf(buf)),
            service::DESCRIPTOR_ID => Descriptor::Service(Service::from_buf(buf)),
            stream_identifier::DESCRIPTOR_ID => {
                Descriptor::StreamIdentifier(StreamIdentifier::from_buf(buf))
            }
            component::DESCRIPTOR_ID => Descriptor::Component(Component::from_buf(buf)),
            terrestrial_delivery_system::DESCRIPTOR_ID => {
                Descriptor::TerrestrialDeliverySystem(TerrestrialDeliverySystem::from_buf(buf))
            }
            subtitling::DESCRIPTOR_ID => Descriptor::Subtitling(Subtitling::from_buf(buf)),
            private_data_specifier::DESCRIPTOR_ID => {
                Descriptor::PrivateDataSpecifier(PrivateDataSpecifier::from_buf(buf))
            }
            data_broadcast_id::DESCRIPTOR_ID => {
                Descriptor::DataBroadcastId(DataBroadcastId::from_buf(buf))
            }
            ac3::DESCRIPTOR_ID => Descriptor::Ac3(Ac3::from_buf(buf)),
            application_signalling::DESCRIPTOR_ID => {
                Descriptor::ApplicationSignalling(ApplicationSignalling::from_buf(buf))
            }
            enhanced_ac3::DESCRIPTOR_ID => Descriptor::EnhancedAc3(EnhancedAc3::from_buf(buf)),
            extension::DESCRIPTOR_ID => Descriptor::Extension(Extension::from_buf(buf)),
            // According to docs, this is "user-defined"... Where are LCN descriptors "officially" defined ???
            logical_channel::DESCRIPTOR_ID => {
                Descriptor::LogicalChannel(LogicalChannel::from_buf(buf))
            }
            _ => Descriptor::_Unknown(UnknownDescriptor {
                descriptor_id,
                raw_data: buf.to_vec(),
            }),
        }
    }

    pub fn read_many(buf: &[u8]) -> Vec<Descriptor> {
        let mut descriptors = Vec::new();

        let mut offset = 0;
        while offset < buf.len() {
            let descriptor_id = buf[offset];
            let length = buf[offset + 1];
            offset += 2;

            let data = &buf[offset..offset + length as usize];
            descriptors.push(Descriptor::read(descriptor_id, data));
            offset += length as usize;
        }

        descriptors
    }

    pub const fn descriptor_id(&self) -> u8 {
        // TODO: Macro
        match self {
            Descriptor::Iso639Language(_) => iso639_language::DESCRIPTOR_ID,
            Descriptor::CarouselIdentifier(_) => carousel_identifier::DESCRIPTOR_ID,
            Descriptor::NetworkName(_) => network_name::DESCRIPTOR_ID,
            Descriptor::ServiceList(_) => service_list::DESCRIPTOR_ID,
            Descriptor::Service(_) => service::DESCRIPTOR_ID,
            Descriptor::Component(_) => component::DESCRIPTOR_ID,
            Descriptor::StreamIdentifier(_) => stream_identifier::DESCRIPTOR_ID,
            Descriptor::Subtitling(_) => subtitling::DESCRIPTOR_ID,
            Descriptor::TerrestrialDeliverySystem(_) => terrestrial_delivery_system::DESCRIPTOR_ID,
            Descriptor::PrivateDataSpecifier(_) => private_data_specifier::DESCRIPTOR_ID,
            Descriptor::DataBroadcastId(_) => data_broadcast_id::DESCRIPTOR_ID,
            Descriptor::Ac3(_) => ac3::DESCRIPTOR_ID,
            Descriptor::ApplicationSignalling(_) => application_signalling::DESCRIPTOR_ID,
            Descriptor::EnhancedAc3(_) => enhanced_ac3::DESCRIPTOR_ID,
            Descriptor::Extension(_) => extension::DESCRIPTOR_ID,
            Descriptor::LogicalChannel(_) => extension::DESCRIPTOR_ID,
            Descriptor::_Unknown(u) => u.descriptor_id,
        }
    }
}

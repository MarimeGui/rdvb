use crate::mpeg::{Packet, descriptors::Descriptor};

/// "Network Information Section - Actual network" table ID, as defined in `EN 300 468 V1.17.1`, p24
pub const ACTUAL_NETWORK_TABLE_ID: u8 = 0x40;

/// NIT describes all services that are available in neighboring area. It contains a list of transponders and associated services
#[derive(Debug, Clone)]
pub struct NetworkInformation {
    pub network_descriptors: Vec<Descriptor>,
    pub elements: Vec<NitElement>,
}

#[derive(Debug, Clone)]
pub struct NitElement {
    pub transport_stream_id: u16,
    pub original_network_id: u16,
    pub transport_descriptors: Vec<Descriptor>,
}

impl NetworkInformation {
    // ETSI EN 300 468 page 27
    pub fn from_packet(packet: &Packet) -> NetworkInformation {
        let mut current_offset = 0;

        let _reserved = packet.data[current_offset] & 0b1111_0000;
        let network_descriptors_length = u16::from_be_bytes([
            packet.data[current_offset] & 0b0000_1111,
            packet.data[current_offset + 1],
        ]);
        current_offset += 2;

        let network_descriptors = Descriptor::read_many(
            &packet.data[current_offset..current_offset + network_descriptors_length as usize],
        );
        current_offset += network_descriptors_length as usize;

        let _reserved = packet.data[current_offset] & 0b1111_0000;
        // let transport_stream_loop_length = u16::from_be_bytes([
        //     packet.data[current_offset] & 0b0000_1111,
        //     packet.data[current_offset + 1],
        // ]);
        current_offset += 2;

        let mut elements = Vec::new();

        // TODO: I'm assuming I have to use payload_len instead of data.len because of CRC32 at the end ? Should check that, maybe have a CRC32 field in Packet
        while (current_offset as u16) < packet.header.payload_len() {
            let transport_stream_id =
                u16::from_be_bytes([packet.data[current_offset], packet.data[current_offset + 1]]);
            current_offset += 2;

            let original_network_id =
                u16::from_be_bytes([packet.data[current_offset], packet.data[current_offset + 1]]);
            current_offset += 2;

            let _reserved = packet.data[current_offset] & 0b1111_0000;
            let transport_descriptors_length = u16::from_be_bytes([
                packet.data[current_offset] & 0b0000_1111,
                packet.data[current_offset + 1],
            ]);
            current_offset += 2;

            let transport_descriptors = Descriptor::read_many(
                &packet.data
                    [current_offset..current_offset + transport_descriptors_length as usize],
            );
            current_offset += transport_descriptors_length as usize;

            elements.push(NitElement {
                transport_stream_id,
                original_network_id,
                transport_descriptors,
            });
        }

        NetworkInformation {
            network_descriptors,
            elements,
        }
    }
}

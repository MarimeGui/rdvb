use crate::mpeg::{Packet, descriptors::Descriptor};

/// "Service Definition Table" fixed packet ID, as defined in `EN 300 468 V1.17.1`, p23
pub const PID: u16 = 0x0011;

/// "Service Description Section - Actual transport stream" table ID, as defined in `EN 300 468 V1.17.1`, p24
pub const ACTUAL_TRANSPORT_TABLE_ID: u8 = 0x42;

#[derive(Debug)]
pub struct ServiceDescription {
    pub original_network_id: u16,
    pub services: Vec<Service>,
}

#[derive(Debug)]
pub struct Service {
    pub service_id: u16,
    pub eit_schedule: bool,
    pub eit_present_following: bool,
    pub running_status: u8,
    pub free_ca_mode: bool,
    pub descriptors: Vec<Descriptor>,
}

impl ServiceDescription {
    // ETSI EN 300 468 page 30
    pub fn from_packet(packet: &Packet) -> Self {
        let original_network_id = u16::from_be_bytes([packet.data[0], packet.data[1]]);
        let _reserved = packet.data[2];

        let mut services = Vec::new();

        let mut offset = 3;
        while offset < (packet.header.payload_len() as usize) {
            let service_id = u16::from_be_bytes([packet.data[offset], packet.data[offset + 1]]);
            let _reserved = packet.data[offset + 2] & 0b1111_1100;
            let eit_schedule = (packet.data[offset + 2] & 0b0000_0010) != 0;
            let eit_present_following = (packet.data[offset + 2] & 0b0000_0001) != 0;
            let running_status = (packet.data[offset + 3] & 0b1110_0000) >> 5;
            let free_ca_mode = (packet.data[offset + 3] & 0b0001_0000) != 0;
            let descriptors_length = u16::from_be_bytes([
                packet.data[offset + 3] & 0b0000_1111,
                packet.data[offset + 4],
            ]);

            offset += 5;

            let descriptors =
                Descriptor::read_many(&packet.data[offset..offset + descriptors_length as usize]);
            offset += descriptors_length as usize;

            services.push(Service {
                service_id,
                eit_schedule,
                eit_present_following,
                running_status,
                free_ca_mode,
                descriptors,
            });
        }

        ServiceDescription {
            original_network_id,
            services,
        }
    }
}

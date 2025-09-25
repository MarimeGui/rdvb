use crate::mpeg::{descriptors::Descriptor, Packet};

/// NIT describes all services that are available in neighboring area. It contains a list of transponders and associated services

#[derive(Debug, Clone)]
pub struct NetworkInformationTable {
    pub network_descriptors: Vec<Descriptor>,
    pub elements: Vec<NitElement>,
}

#[derive(Debug, Clone)]
pub struct NitElement {
    pub transport_stream_id: u16,
    pub original_network_id: u16,
    pub transport_descriptors: Vec<Descriptor>,
}

// ETSI EN 300 468 page 27
pub fn parse_nit(packet: &Packet) -> NetworkInformationTable {
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
            &packet.data[current_offset..current_offset + transport_descriptors_length as usize],
        );
        current_offset += transport_descriptors_length as usize;

        elements.push(NitElement {
            transport_stream_id,
            original_network_id,
            transport_descriptors,
        });
    }

    NetworkInformationTable {
        network_descriptors,
        elements,
    }
}

pub fn decode_nit(packet: &Packet) {
    let _reserved = packet.data[0] & 0b1111_0000;
    let network_descriptors_length =
        u16::from_be_bytes([packet.data[0] & 0b0000_1111, packet.data[1]]);
    let mut current_offset = 2;

    let extra = &packet.data[current_offset..current_offset + network_descriptors_length as usize];

    // TODO: Copy pasta
    let mut offset = 0;
    while offset < extra.len() {
        let descriptor_id = extra[offset];
        let len = extra[offset + 1];
        offset += 2;

        match descriptor_id {
            0x40 => {
                // let raw_text = &extra[offset..offset + (len as usize)];
                offset += len as usize;
                // let name = String::from_utf8(raw_text.to_vec()).unwrap();
            }
            x => {
                panic!("Unexpected descriptor 0x{:X} for network descriptors", x)
            }
        }
    }

    current_offset += extra.len();

    let _reserved = (packet.data[current_offset] & 0b1111_0000) >> 4;
    // let transport_stream_loop_length = (((packet.data[current_offset] as u16) & 0b0000_1111) << 8)
    //     | (packet.data[current_offset + 1] as u16);
    current_offset += 2;

    while (current_offset as u16) < packet.header.payload_len() {
        // let transport_stream_id =
        //     u16::from_be_bytes([packet.data[current_offset], packet.data[current_offset + 1]]);
        // let original_network_id = u16::from_be_bytes([
        //     packet.data[current_offset + 2],
        //     packet.data[current_offset + 3],
        // ]);
        let _reserved = packet.data[current_offset + 4] & 0b1111_0000;
        let transport_descriptors_length = u16::from_be_bytes([
            packet.data[current_offset + 4] & 0b0000_1111,
            packet.data[current_offset + 5],
        ]);
        current_offset += 6;

        // println!("        Transport Stream ID: {}, Original Network ID: {}", transport_stream_id, original_network_id);

        let extra =
            &packet.data[current_offset..current_offset + transport_descriptors_length as usize];

        // TODO: Copy pasta
        let mut offset = 0;
        while offset < extra.len() {
            let descriptor_id = extra[offset];
            let len = extra[offset + 1];
            offset += 2;

            match descriptor_id {
                0x41 => {
                    // println!("            - Service list descriptor");

                    offset += len as usize;
                }
                0x5A => {
                    // ETSI EN 300 468 page 61
                    // println!("            - Terrestrial delivery system descriptor");

                    // let center_frequency = u32::from_be_bytes([
                    //     extra[offset],
                    //     extra[offset + 1],
                    //     extra[offset + 2],
                    //     extra[offset + 3],
                    // ]);
                    // let bandwidth = (extra[offset + 4] & 0b1110_0000) >> 5;
                    // let priority = (extra[offset + 4] & 0b0001_0000) != 0;
                    // let time_slicing_indicator = (extra[offset + 4] & 0b0000_1000) != 0;
                    // let mpe_fec_indicator = (extra[offset + 4] & 0b0000_0100) != 0;
                    let _reserved = extra[offset + 4] & 0b0000_0011;
                    // let constellation = (extra[offset + 5] & 0b1100_0000) >> 6;
                    // let hierarchy_information = (extra[offset + 5] & 0b0011_1000) >> 3;
                    // let code_rate_hp_stream = extra[offset + 5] & 0b0000_0111;
                    // let code_rate_lp_stream = (extra[offset + 6] & 0b1110_0000) >> 5;
                    // let guard_interval = (extra[offset + 6] & 0b0001_1000) >> 3;
                    // let transmission_mode = (extra[offset + 6] & 0b0000_0110) >> 1;
                    // let other_frequency_flag = (extra[offset + 6] & 0b0000_0001) != 0;
                    let _reserved = u32::from_be_bytes([
                        extra[offset + 7],
                        extra[offset + 8],
                        extra[offset + 9],
                        extra[offset + 10],
                    ]);

                    offset += len as usize;
                }
                0x5F => {
                    // println!("            - Private data specifier descriptor");
                    offset += len as usize;
                }
                0x83 => {
                    // TODO: Where is this actually defined ?
                    // TODO: Following w_scan2 implementation, seems like results are wrong

                    // println!("            - Logical channel descriptor");
                    let begin = offset;
                    while offset < begin + len as usize {
                        // let service_id =
                        //     u16::from_be_bytes([packet.data[offset], packet.data[offset + 1]]);
                        // let visible_service = (packet.data[offset + 2] & 0b1000_0000) != 0;
                        // let logical_channel_number = u16::from_be_bytes([
                        //     packet.data[offset + 2] & 0b0000_0011,
                        //     packet.data[offset + 3],
                        // ]);

                        offset += 4;
                    }
                }
                _ => {
                    // panic!("Unexpected descriptor for transport descriptors")
                    offset += len as usize;
                }
            }
        }

        current_offset += transport_descriptors_length as usize;
    }
}

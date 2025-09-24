// ISO/IEC 13818-1 page 61

use crate::mpeg::Packet;

#[derive(Debug)]
pub struct PatElement {
    pub program_number: u16,
    pub value: PatValue,
}

#[derive(Debug)]
pub enum PatValue {
    Network(u16),
    ProgramMap(u16),
}

/// Program Association Table
pub fn parse_pat(packet: &Packet) -> Vec<PatElement> {
    // let transport_stream_id = packet.header.identifier;

    let mut elements = Vec::new();

    let mut current_offset = 0;
    // Removing 5 bytes after the section length field in header and 4 bytes of CRC.
    while (current_offset as u16) < packet.header.payload_len() {
        let program_number =
            u16::from_be_bytes([packet.data[current_offset], packet.data[current_offset + 1]]);
        current_offset += 2;
        let _another_reserved = packet.data[current_offset] & 0b1110_0000;
        let value = u16::from_be_bytes([
            packet.data[current_offset] & 0b0001_1111,
            packet.data[current_offset + 1],
        ]);
        current_offset += 2;

        elements.push(PatElement {
            program_number,
            value: {
                if program_number == 0 {
                    // TODO: Apparently if this is 16 this really isn't the network ID
                    PatValue::Network(value)
                } else {
                    PatValue::ProgramMap(value)
                }
            },
        });
    }

    // CRC here

    elements
}

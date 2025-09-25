use crate::mpeg::ServiceType;

pub const DESCRIPTOR_ID: u8 = 0x41;

#[derive(Debug, Clone)]
pub struct ServiceList {
    pub services: Vec<ServiceListDescriptorElement>,
}

#[derive(Debug, Clone)]
pub struct ServiceListDescriptorElement {
    /// Same as program number in program map except for 0x04, 0x18, 0x1B (NVOD services) (from ETSI EN 300 468)
    pub service_id: u16,
    pub service_type: ServiceType,
}

impl ServiceList {
    pub fn from_buf(buf: &[u8]) -> ServiceList {
        let mut services = Vec::new();

        let mut offset = 0;
        while offset < buf.len() {
            let service_id = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
            let service_type = ServiceType::from_byte(buf[offset + 2]);
            offset += 3;
            services.push(ServiceListDescriptorElement {
                service_id,
                service_type,
            });
        }

        ServiceList { services }
    }
}

pub const DESCRIPTOR_ID: u8 = 0x5A;

#[derive(Debug, Clone)]
pub struct TerrestrialDeliverySystem {
    pub center_frequency: i32,
    pub bandwidth: u8,
    pub priority: bool,
    pub time_slicing_indicator: bool,
    pub mpe_fec_indicator: bool,
    pub constellation: u8,
    pub hierarchy_information: u8,
    pub code_rate_hp_stream: u8,
    pub code_rate_lp_stream: u8,
    pub guard_interval: u8,
    pub transmission_mode: u8,
    pub other_frequency_flag: bool,
}

impl TerrestrialDeliverySystem {
    pub fn from_buf(buf: &[u8]) -> TerrestrialDeliverySystem {
        let center_frequency = i32::from_be_bytes([buf[0], buf[2], buf[1], buf[3]]);
        let bandwidth = (buf[4] & 0b1110_0000) >> 5;
        let priority = (buf[4] & 0b0001_0000) != 0;
        let time_slicing_indicator = (buf[4] & 0b0000_1000) != 0;
        let mpe_fec_indicator = (buf[4] & 0b0000_0100) != 0;
        let _reserved = buf[4] & 0b0000_0011;
        let constellation = (buf[5] & 0b1100_0000) >> 6;
        let hierarchy_information = (buf[5] & 0b0011_1000) >> 3;
        let code_rate_hp_stream = buf[5] & 0b0000_0111;
        let code_rate_lp_stream = (buf[6] & 0b1110_0000) >> 5;
        let guard_interval = (buf[6] & 0b0001_1000) >> 3;
        let transmission_mode = (buf[6] & 0b0000_0110) >> 1;
        let other_frequency_flag = (buf[6] & 0b0000_0001) != 0;
        let _reserved = u32::from_be_bytes([buf[7], buf[8], buf[9], buf[10]]);

        TerrestrialDeliverySystem {
            center_frequency,
            bandwidth,
            priority,
            time_slicing_indicator,
            mpe_fec_indicator,
            constellation,
            hierarchy_information,
            code_rate_hp_stream,
            code_rate_lp_stream,
            guard_interval,
            transmission_mode,
            other_frequency_flag,
        }
    }
}

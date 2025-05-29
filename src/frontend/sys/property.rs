use std::ffi::{c_int, c_void};

//
// ----- Commands

// Specifically setting this enum to u32 as it is just a collection of defines in header file, and will only be used in cmd field in DtvProperty.
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Command {
    DTV_UNDEFINED = 0,
    DTV_TUNE = 1,
    DTV_CLEAR = 2,
    /// Frequency of the digital TV transponder/channel.
    ///
    /// Note:
    ///
    /// 1. For satellite delivery systems, the frequency is in kHz.
    /// 2. For cable and terrestrial delivery systems, the frequency is in Hz.
    /// 3. On most delivery systems, the frequency is the center frequency of the transponder/channel. The exception is for ISDB-T, where the main carrier has a 1/7 offset from the center.
    /// 4. For ISDB-T, the channels are usually transmitted with an offset of about 143kHz. E.g. a valid frequency could be 474,143 kHz. The stepping is bound to the bandwidth of the channel which is typically 6MHz.
    /// 5. In ISDB-Tsb, the channel consists of only one or three segments the frequency step is 429kHz, 3*429 respectively.
    ///
    /// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/fe_property_parameters.html#dtv-frequency))
    DTV_FREQUENCY = 3,
    /// Specifies the frontend modulation type for delivery systems that supports more multiple modulations.
    ///
    /// The modulation can be one of the types defined by enum fe_modulation.
    ///
    /// Most of the digital TV standards offers more than one possible modulation type.
    ///
    /// The table below presents a summary of the types of modulation types supported by each delivery system, as currently defined by specs.
    ///
    /// | Standard         | Modulation types                                       |
    /// |------------------|--------------------------------------------------------|
    /// | ATSC (version 1) | 8-VSB and 16-VSB.                                      |
    /// | DMTB             | 4-QAM, 16-QAM, 32-QAM, 64-QAM and 4-QAM-NR.            |
    /// | DVB-C Annex A/C  | 16-QAM, 32-QAM, 64-QAM and 256-QAM.                    |
    /// | DVB-C Annex B    | 64-QAM.                                                |
    /// | DVB-C2           | QPSK, 16-QAM, 64-QAM, 256-QAM, 1024-QAM and 4096-QAM.  |
    /// | DVB-T            | QPSK, 16-QAM and 64-QAM.                               |
    /// | DVB-T2           | QPSK, 16-QAM, 64-QAM and 256-QAM.                      |
    /// | DVB-S            | No need to set. It supports only QPSK.                 |
    /// | DVB-S2           | QPSK, 8-PSK, 16-APSK and 32-APSK.                      |
    /// | DVB-S2X          | 8-APSK-L, 16-APSK-L, 32-APSK-L, 64-APSK and 64-APSK-L. |
    /// | ISDB-T           | QPSK, DQPSK, 16-QAM and 64-QAM.                        |
    /// | ISDB-S           | 8-PSK, QPSK and BPSK.                                  |
    ///
    /// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/fe_property_parameters.html#dtv-modulation))
    DTV_MODULATION = 4,
    /// Bandwidth for the channel, in HZ.
    ///
    /// Should be set only for terrestrial delivery systems.
    ///
    /// | Terrestrial Standard | Possible values for bandwidth                             |
    /// |----------------------|-----------------------------------------------------------|
    /// | ATSC (version 1)     | No need to set. It is always 6MHz.                        |
    /// | DMTB                 | No need to set. It is always 8MHz.                        |
    /// | DVB-T                | 6MHz, 7MHz and 8MHz.                                      |
    /// | DVB-T2               | 1.172 MHz, 5MHz, 6MHz, 7MHz, 8MHz and 10MHz               |
    /// | ISDB-T               | 5MHz, 6MHz, 7MHz and 8MHz, although most places use 6MHz. |
    ///
    /// Note:
    ///
    /// 1. For ISDB-Tsb, the bandwidth can vary depending on the number of connected segments.\
    ///    It can be easily derived from other parameters (DTV_ISDBT_SB_SEGMENT_IDX, DTV_ISDBT_SB_SEGMENT_COUNT).
    /// 2. On Satellite and Cable delivery systems, the bandwidth depends on the symbol rate. So, the Kernel will silently ignore any setting [DTV_BANDWIDTH_HZ](BandwidthHz). I will however fill it back with a bandwidth estimation.\
    ///    Such bandwidth estimation takes into account the symbol rate set with [DTV_SYMBOL_RATE](SymbolRate), and the rolloff factor, with is fixed for DVB-C and DVB-S.\
    ///    For DVB-S2, the rolloff should also be set via [DTV_ROLLOFF](Rolloff).
    ///
    /// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/fe_property_parameters.html#dtv-bandwidth-hz))
    DTV_BANDWIDTH_HZ = 5,
    /// Specifies if the frontend should do spectral inversion or not.
    ///
    /// The acceptable values are defined by [fe_spectral_inversion](FeSpectralInversion).
    ///
    /// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/fe_property_parameters.html#dtv-inversion))
    DTV_INVERSION = 6,
    DTV_DISEQC_MASTER = 7,
    DTV_SYMBOL_RATE = 8,
    DTV_INNER_FEC = 9,
    DTV_VOLTAGE = 10,
    DTV_TONE = 11,
    DTV_PILOT = 12,
    DTV_ROLLOFF = 13,
    DTV_DISEQC_SLAVE_REPLY = 14,

    //
    // ----- Basic enumeration set for querying unlimited capabilities
    DTV_FE_CAPABILITY_COUNT = 15,
    DTV_FE_CAPABILITY = 16,
    DTV_DELIVERY_SYSTEM = 17,

    //
    // ----- ISDB-T and ISDB-Tsb
    DTV_ISDBT_PARTIAL_RECEPTION = 18,
    DTV_ISDBT_SOUND_BROADCASTING = 19,
    DTV_ISDBT_SB_SUBCHANNEL_ID = 20,
    DTV_ISDBT_SB_SEGMENT_IDX = 21,
    DTV_ISDBT_SB_SEGMENT_COUNT = 22,
    DTV_ISDBT_LAYERA_FEC = 23,
    DTV_ISDBT_LAYERA_MODULATION = 24,
    DTV_ISDBT_LAYERA_SEGMENT_COUNT = 25,
    DTV_ISDBT_LAYERA_TIME_INTERLEAVING = 26,
    DTV_ISDBT_LAYERB_FEC = 27,
    DTV_ISDBT_LAYERB_MODULATION = 28,
    DTV_ISDBT_LAYERB_SEGMENT_COUNT = 29,
    DTV_ISDBT_LAYERB_TIME_INTERLEAVING = 30,
    DTV_ISDBT_LAYERC_FEC = 31,
    DTV_ISDBT_LAYERC_MODULATION = 32,
    DTV_ISDBT_LAYERC_SEGMENT_COUNT = 33,
    DTV_ISDBT_LAYERC_TIME_INTERLEAVING = 34,
    DTV_API_VERSION = 35,
    DTV_CODE_RATE_HP = 36,
    DTV_CODE_RATE_LP = 37,
    DTV_GUARD_INTERVAL = 38,
    DTV_TRANSMISSION_MODE = 39,
    DTV_HIERARCHY = 40,
    DTV_ISDBT_LAYER_ENABLED = 41,
    DTV_STREAM_ID = 42,
    // DTV_ISDBS_TS_ID_LEGACY	DTV_STREAM_ID
    DTV_DVBT2_PLP_ID_LEGACY = 43,
    /// Use to figure out what transmission systems (DVB-S, DVB-T...) the frontend can work with.
    ///
    /// From [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/fe_property_parameters.html#dtv-enum-delsys):
    ///
    /// "A Multi standard frontend needs to advertise the delivery systems provided.
    /// Applications need to enumerate the provided delivery systems,
    /// before using any other operation with the frontend.
    /// Prior to it’s introduction,
    /// FE_GET_INFO was used to determine a frontend type.
    /// A frontend which provides more than a single delivery system,
    /// FE_GET_INFO doesn’t help much.
    /// Applications which intends to use a multistandard frontend must
    /// enumerate the delivery systems associated with it,
    /// rather than trying to use FE_GET_INFO.
    /// In the case of a legacy frontend,
    /// the result is just the same as with FE_GET_INFO,
    /// but in a more structured format"
    DTV_ENUM_DELSYS = 44,

    //
    // ----- ATSC-MH
    DTV_ATSCMH_FIC_VER = 45,
    DTV_ATSCMH_PARADE_ID = 46,
    DTV_ATSCMH_NOG = 47,
    DTV_ATSCMH_TNOG = 48,
    DTV_ATSCMH_SGN = 49,
    DTV_ATSCMH_PRC = 50,
    DTV_ATSCMH_RS_FRAME_MODE = 51,
    DTV_ATSCMH_RS_FRAME_ENSEMBLE = 52,
    DTV_ATSCMH_RS_CODE_MODE_PRI = 53,
    DTV_ATSCMH_RS_CODE_MODE_SEC = 54,
    DTV_ATSCMH_SCCC_BLOCK_MODE = 55,
    DTV_ATSCMH_SCCC_CODE_MODE_A = 56,
    DTV_ATSCMH_SCCC_CODE_MODE_B = 57,
    DTV_ATSCMH_SCCC_CODE_MODE_C = 58,
    DTV_ATSCMH_SCCC_CODE_MODE_D = 59,
    DTV_INTERLEAVING = 60,
    DTV_LNA = 61,

    //
    // ----- Quality parameters
    DTV_STAT_SIGNAL_STRENGTH = 62,
    DTV_STAT_CNR = 63,
    DTV_STAT_PRE_ERROR_BIT_COUNT = 64,
    DTV_STAT_PRE_TOTAL_BIT_COUNT = 65,
    DTV_STAT_POST_ERROR_BIT_COUNT = 66,
    DTV_STAT_POST_TOTAL_BIT_COUNT = 67,
    DTV_STAT_ERROR_BLOCK_COUNT = 68,
    DTV_STAT_TOTAL_BLOCK_COUNT = 69,

    //
    // ------ Physical layer scrambling
    DTV_SCRAMBLING_SEQUENCE_INDEX = 70,
}

//
// ----- Structs

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DtvProperties {
    pub num: u32,
    pub props: *mut DtvProperty,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DtvProperty {
    pub cmd: u32,
    pub reserved: [u32; 3],
    pub u: DtvPropertyUnion,
    pub result: c_int,
}

impl DtvProperty {
    pub fn new_empty(cmd: Command) -> DtvProperty {
        // TODO: Is setting DtvPropertyUnion data to 0 enough ?
        DtvProperty {
            cmd: cmd as u32,
            reserved: [0; 3],
            u: DtvPropertyUnion { data: 0 },
            result: 0,
        }
    }

    pub fn new_data(cmd: Command, data: u32) -> DtvProperty {
        DtvProperty {
            cmd: cmd as u32,
            reserved: [0; 3],
            u: DtvPropertyUnion { data },
            result: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DtvPropertyUnion {
    pub data: u32,
    pub st: DtvFeStats,
    pub buffer: DtvPropertyABuffer,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DtvFeStats {
    pub len: u8,
    pub stat: [DtvStats; 4],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DtvStats {
    pub scale: u8,
    pub __bindgen_anon_1: DtvStatsUnion,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union DtvStatsUnion {
    pub uvalue: u64,
    pub svalue: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DtvPropertyABuffer {
    pub data: [u8; 32],
    pub len: u32,
    pub reserved1: [u32; 3],
    pub reserved2: *mut c_void,
}

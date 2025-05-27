use std::{
    ffi::{c_char, c_int, c_uint, c_void},
    fmt,
    mem::MaybeUninit,
    os::fd::{AsRawFd, BorrowedFd},
};

use nix::ioctl_read;

//
// ----- IOCTLs

pub(crate) const FE_TYPE: u8 = b'o';

const FE_GET_INFO: u8 = 61;
ioctl_read!(fe_get_info, FE_TYPE, FE_GET_INFO, DvbFrontendInfo);

const FE_READ_STATUS: u8 = 69;
ioctl_read!(fe_read_status, FE_TYPE, FE_READ_STATUS, c_uint); // Maps to FeStatus struct for bits

// const FE_SET_PROPERTY: u8 = 82;
// ioctl_write_ptr!(fe_set_info, FE_TYPE, FE_SET_PROPERTY, DtvProperties);

const FE_GET_PROPERTY: u8 = 83;
ioctl_read!(fe_get_property, FE_TYPE, FE_GET_PROPERTY, DtvProperties);

//
// ----- Simplified IOCTLs

// TODO: Return error
pub fn get_info(fd: BorrowedFd) -> Option<DvbFrontendInfo> {
    let mut info = MaybeUninit::uninit();
    unsafe { fe_get_info(fd.as_raw_fd(), info.as_mut_ptr()) }.unwrap();
    let info = unsafe { info.assume_init() };
    Some(info)
}

pub fn read_status(fd: BorrowedFd) -> Option<c_uint> {
    let mut status = MaybeUninit::uninit();
    unsafe { fe_read_status(fd.as_raw_fd(), status.as_mut_ptr()) }.unwrap();
    let status = unsafe { status.assume_init() };
    Some(status)
}

pub fn get_properties_raw(fd: BorrowedFd, count: usize, ptr: *mut DtvProperty) -> Option<()> {
    if count == 0 {
        return Some(());
    }

    let mut properties = DtvProperties {
        num: count as u32,
        props: ptr,
    };

    unsafe { fe_get_property(fd.as_raw_fd(), &mut properties as *mut DtvProperties) }.unwrap();

    Some(())
}

//
// ----- Get/Set Property commands

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum PropertyCommands {
    Tune = 1,
    Clear = 2,
    Frequency = 3,
    Modulation = 4,
    BandwidthHz = 5,
    Inversion = 6,
    DiseqcMaster = 7,
    SymbolRate = 8,
    InnerFec = 9,
    Voltage = 10,
    Tone = 11,
    Pilot = 12,
    Rolloff = 13,
    DiseqcSlaveReply = 14,

    FeCapabilityCount = 15,
    FeCapability = 16,
    DeliverySystem = 17,

    IsdbtPartialReception = 18,
    IsdbtSoundBroadcasting = 19,

    IsdbtSbSubchannelId = 20,
    IsdbtSbSegmentIdx = 21,
    IsdbtSbSegmentCount = 22,

    IsdbtLayerAFec = 23,
    IsdbtLayerAModulation = 24,
    // Some missing
    ApiVersion = 35,

    CodeRateHp = 36,
    CodeRateLp = 37,
    GuardInterval = 38,
    TransmissionMode = 39,
    Hierarchy = 40,

    IsdbtLayerEnabled = 41,

    StreamId = 42,
    // IsdbdTsIdLegacy
    // DvbT2PlpIdLegacy
    EnumDelSys = 44,

    // ATSC
    Interleaving = 60,
    Lna = 61,

    StatSignalStrength = 62,
    StatCnr = 63,
    StatPreErrorBitCount = 64,
    StatPreTotalBitCount = 65,
    StatPostErrorBitCount = 66,
    StatErrorBlockCount = 68,
    StatTotalBlockCount = 69,

    ScramblingSequenceIndex = 70,
}

//
// ----- C structs

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DvbFrontendInfo {
    pub name: [c_char; 128],
    pub type_: FeType,
    pub frequency_min: u32,
    pub frequency_max: u32,
    pub frequency_stepsize: u32,
    pub frequency_tolerance: u32,
    pub symbol_rate_min: u32,
    pub symbol_rate_max: u32,
    pub symbol_rate_tolerance: u32,
    pub notifier_delay: u32,
    pub caps: FeCaps,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum FeType {
    Qpsk,
    Qam,
    Ofdm,
    Atsc,
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FeCaps(u32);
// TODO: FeCaps bits
impl FeCaps {}

//
// ----- Status

pub struct FeStatus(u32);

impl From<c_uint> for FeStatus {
    fn from(value: c_uint) -> Self {
        FeStatus(value)
    }
}

impl fmt::Debug for FeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeStatus")
            .field("Has Signal", &self.has_signal())
            .field("Has Carrier", &self.has_carrier())
            .field("Has Viterbi", &self.has_viterbi())
            .field("Has Sync", &self.has_sync())
            .field("Has Lock", &self.has_lock())
            .field("Timed out", &self.timed_out())
            .field("Reinit", &self.reinit())
            .finish()
    }
}

impl FeStatus {
    // const NONE: c_uint = 0;
    const HAS_SIGNAL_BIT: c_uint = 1;
    const HAS_CARRIER_BIT: c_uint = 2;
    const HAS_VITERBI_BIT: c_uint = 4;
    const HAS_SYNC_BIT: c_uint = 8;
    const HAS_LOCK_BIT: c_uint = 16;
    const TIMEDOUT_BIT: c_uint = 32;
    const REINIT_BIT: c_uint = 64;

    /// "The frontend doesn’t have any kind of lock. That’s the initial frontend status"
    pub fn none(&self) -> bool {
        self.0 == 0
    }

    /// "Has found something above the noise level."
    pub fn has_signal(&self) -> bool {
        (self.0 & Self::HAS_SIGNAL_BIT) != 0
    }

    /// "Has found a signal."
    pub fn has_carrier(&self) -> bool {
        (self.0 & Self::HAS_CARRIER_BIT) != 0
    }

    /// "FEC inner coding (Viterbi, LDPC or other inner code). is stable."
    pub fn has_viterbi(&self) -> bool {
        (self.0 & Self::HAS_VITERBI_BIT) != 0
    }

    /// "Synchronization bytes was found."
    pub fn has_sync(&self) -> bool {
        (self.0 & Self::HAS_SYNC_BIT) != 0
    }

    /// "Digital TV were locked and everything is working."
    pub fn has_lock(&self) -> bool {
        (self.0 & Self::HAS_LOCK_BIT) != 0
    }

    /// "Fo lock within the last about 2 seconds."
    pub fn timed_out(&self) -> bool {
        (self.0 & Self::TIMEDOUT_BIT) != 0
    }

    /// "Frontend was reinitialized, application is recommended to reset DiSEqC, tone and parameters."
    pub fn reinit(&self) -> bool {
        (self.0 & Self::REINIT_BIT) != 0
    }
}

//
// ----- Data

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

/// Filled out by DTV_ENUM_DELSYS
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FeDeliverySystem(pub u32);

impl FeDeliverySystem {}

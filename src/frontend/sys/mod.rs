pub mod ioctl;
pub mod property;

use std::{
    ffi::{c_char, c_uint},
    fmt,
};

use enum_from_discriminant_derive::TryFromDiscriminant;

//
// ----- Frontend Info

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

//
// ----- Status

// TODO: Replace with https://github.com/meithecatte/enumflags2 or similar
// TODO: Is FeStatus actually u32 ?
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
    // const NONE: u32 = 0;
    const HAS_SIGNAL_BIT: u32 = 1;
    const HAS_CARRIER_BIT: u32 = 2;
    const HAS_VITERBI_BIT: u32 = 4;
    const HAS_SYNC_BIT: u32 = 8;
    const HAS_LOCK_BIT: u32 = 16;
    const TIMEDOUT_BIT: u32 = 32;
    const REINIT_BIT: u32 = 64;

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
// ----- Data used in properties (and more)

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum FeType {
    FE_QPSK,
    FE_QAM,
    FE_OFDM,
    FE_ATSC,
}

// TODO: Is FeCaps actually u32 ?
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FeCaps(u32);
// TODO: FeCaps bits
impl FeCaps {}

/// Type of the delivery system
///
/// (from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/frontend-header.html#c.fe_delivery_system))
#[repr(C)]
#[derive(Debug, Copy, Clone, TryFromDiscriminant)]
#[allow(non_camel_case_types)]
pub enum FeDeliverySystem {
    /// Undefined standard. Generally, indicates an error
    UNDEFINED,
    /// Cable TV: DVB-C following ITU-T J.83 Annex A spec
    DVBC_ANNEX_A,
    /// Cable TV: DVB-C following ITU-T J.83 Annex B spec (ClearQAM)
    DVBC_ANNEX_B,
    /// Terrestrial TV: DVB-T
    DVBT,
    /// Satellite TV: DSS (not fully supported)
    DSS,
    /// Satellite TV: DVB-S
    DVBS,
    /// Satellite TV: DVB-S2 and DVB-S2X
    DVBS2,
    /// Terrestrial TV (mobile): DVB-H (standard deprecated)
    DVBH,
    /// Terrestrial TV: ISDB-T
    ISDBT,
    /// Satellite TV: ISDB-S
    ISDBS,
    /// Cable TV: ISDB-C (no drivers yet)
    ISDBC,
    /// Terrestrial TV: ATSC
    ATSC,
    /// Terrestrial TV (mobile): ATSC-M/H
    ATSCMH,
    /// Terrestrial TV: DTMB
    DTMB,
    /// Terrestrial TV (mobile): CMMB (not fully supported)
    CMMB,
    /// Digital audio: DAB (not fully supported)
    DAB,
    /// Terrestrial TV: DVB-T2
    DVBT2,
    /// Satellite TV: DVB-S Turbo
    TURBO,
    /// Cable TV: DVB-C following ITU-T J.83 Annex C spec
    DVBC_ANNEX_C,
    /// Cable TV: DVB-C2
    DVBC2,
}

/// Type of modulation/constellation
///
/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/frontend-header.html#c.fe_modulation))
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum FeModulation {
    /// QPSK modulation
    QPSK,
    /// 16-QAM modulation
    QAM_16,
    /// 32-QAM modulation
    QAM_32,
    /// 64-QAM modulation
    QAM_64,
    /// 128-QAM modulation
    QAM_128,
    /// 256-QAM modulation
    QAM_256,
    /// Autodetect QAM modulation
    QAM_AUTO,
    /// 8-VSB modulation
    VSB_8,
    /// 16-VSB modulation
    VSB_16,
    /// 8-PSK modulation
    PSK_8,
    /// 16-APSK modulation
    APSK_16,
    /// 32-APSK modulation
    APSK_32,
    /// DQPSK modulation
    DQPSK,
    /// 4-QAM-NR modulation
    QAM_4_NR,
    /// 1024-QAM modulation
    QAM_1024,
    /// 4096-QAM modulation
    QAM_4096,
    /// 8APSK-L modulation
    APSK_8_L,
    /// 16APSK-L modulation
    APSK_16_L,
    /// 32APSK-L modulation
    APSK_32_L,
    /// 64APSK modulation
    APSK_64,
    /// 64APSK-L modulation
    APSK_64_L,
}

/// Type of inversion band
///
/// This parameter indicates if spectral inversion should be presumed or not.
/// In the automatic setting (``INVERSION_AUTO``) the hardware will try to figure out the correct setting by itself.
/// If the hardware doesn't support, the %dvb_frontend will try to lock at the carrier first with inversion off.
/// If it fails, it will try to enable inversion.
///
/// (taken from [linux/dvb/frontend.h](https://github.com/gjasny/v4l-utils/blob/c4cb1d1bb6960679e1272493102c6dcf4cec76e7/include/linux/dvb/frontend.h#L248))
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum FeSpectralInversion {
    /// Don't do spectral band inversion.
    INVERSION_OFF,
    /// Do spectral band inversion.
    INVERSION_ON,
    /// Autodetect spectral band inversion.
    INVERSION_AUTO,
}

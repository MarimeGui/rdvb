pub mod ioctl;
pub mod property;

use std::{
    ffi::{c_char, c_uint},
    fmt,
};

use enum_from_discriminant_derive::TryFromDiscriminant;

//
// ----- Constants

pub const DTV_IOCTL_MAX_MSGS: usize = 64;

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
// TODO: This really isn't sys anymore. Either I can re-export the type or move the entire thing up
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
#[derive(Debug, Copy, Clone, TryFromDiscriminant, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[derive(Debug, Copy, Clone, TryFromDiscriminant)]
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

/// Guard interval
///
/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/frontend-header.html#c.fe_guard_interval))
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum FeGuardInterval {
    /// Guard interval 1/32
    GUARD_INTERVAL_1_32,
    /// Guard interval 1/16
    GUARD_INTERVAL_1_16,
    /// Guard interval 1/8
    GUARD_INTERVAL_1_8,
    /// Guard interval 1/4
    GUARD_INTERVAL_1_4,
    /// Autodetect the guard interval
    GUARD_INTERVAL_AUTO,
    /// Guard interval 1/128
    GUARD_INTERVAL_1_128,
    /// Guard interval 19/128
    GUARD_INTERVAL_19_128,
    /// Guard interval 19/256
    GUARD_INTERVAL_19_256,
    /// PN length 420 (1/4)
    GUARD_INTERVAL_PN420,
    /// PN length 595 (1/6)
    GUARD_INTERVAL_PN595,
    /// PN length 945 (1/9)
    GUARD_INTERVAL_PN945,
    /// Guard interval 1/64
    GUARD_INTERVAL_1_64,
}

/// Transmission mode
///
/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/frontend-header.html#c.fe_transmit_mode))
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum FeTransmitMode {
    /// Transmission mode 2K
    TRANSMISSION_MODE_2K,
    /// Transmission mode 8K
    TRANSMISSION_MODE_8K,
    /// Autodetect transmission mode. The hardware will try to find the correct FFT-size (if capable) to fill in the missing parameters.
    TRANSMISSION_MODE_AUTO,
    /// Transmission mode 4K
    TRANSMISSION_MODE_4K,
    /// Transmission mode 1K
    TRANSMISSION_MODE_1K,
    /// Transmission mode 16K
    TRANSMISSION_MODE_16K,
    /// Transmission mode 32K
    TRANSMISSION_MODE_32K,
    /// Single Carrier (C=1) transmission mode (DTMB only)
    TRANSMISSION_MODE_C1,
    /// Multi Carrier (C=3780) transmission mode (DTMB only)
    TRANSMISSION_MODE_C3780,
}

/// Type of Forward Error Correction (FEC)
///
/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/frontend-header.html#c.fe_code_rate))
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum FeCodeRate {
    /// No Forward Error Correction Code
    FEC_NONE = 0,
    /// Forward Error Correction Code 1/2
    FEC_1_2,
    /// Forward Error Correction Code 2/3
    FEC_2_3,
    /// Forward Error Correction Code 3/4
    FEC_3_4,
    /// Forward Error Correction Code 4/5
    FEC_4_5,
    /// Forward Error Correction Code 5/6
    FEC_5_6,
    /// Forward Error Correction Code 6/7
    FEC_6_7,
    /// Forward Error Correction Code 7/8
    FEC_7_8,
    /// Forward Error Correction Code 8/9
    FEC_8_9,
    /// Autodetect Error Correction Code
    FEC_AUTO,
    /// Forward Error Correction Code 3/5
    FEC_3_5,
    /// Forward Error Correction Code 9/10
    FEC_9_10,
    /// Forward Error Correction Code 2/5
    FEC_2_5,
    /// Forward Error Correction Code 1/3
    FEC_1_3,
    /// Forward Error Correction Code 1/4
    FEC_1_4,
    /// Forward Error Correction Code 5/9
    FEC_5_9,
    /// Forward Error Correction Code 7/9
    FEC_7_9,
    /// Forward Error Correction Code 8/15
    FEC_8_15,
    /// Forward Error Correction Code 11/15
    FEC_11_15,
    /// Forward Error Correction Code 13/18
    FEC_13_18,
    /// Forward Error Correction Code 9/20
    FEC_9_20,
    /// Forward Error Correction Code 11/20
    FEC_11_20,
    /// Forward Error Correction Code 23/36
    FEC_23_36,
    /// Forward Error Correction Code 25/36
    FEC_25_36,
    /// Forward Error Correction Code 13/45
    FEC_13_45,
    /// Forward Error Correction Code 26/45
    FEC_26_45,
    /// Forward Error Correction Code 28/45
    FEC_28_45,
    /// Forward Error Correction Code 32/45
    FEC_32_45,
    /// Forward Error Correction Code 77/90
    FEC_77_90,
    /// Forward Error Correction Code 11/45
    FEC_11_45,
    /// Forward Error Correction Code 4/15
    FEC_4_15,
    /// Forward Error Correction Code 14/45
    FEC_14_45,
    /// Forward Error Correction Code 7/15
    FEC_7_15,
}

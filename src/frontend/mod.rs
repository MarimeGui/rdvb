pub mod properties;
pub mod sys;

use std::{
    ffi::{CStr, c_char},
    mem::MaybeUninit,
    os::fd::{AsFd, OwnedFd},
    path::Path,
};

use nix::{
    fcntl::{OFlag, open},
    sys::stat::Mode,
};

use crate::utils::ValueBounds;
use sys::{
    DtvProperty, DvbFrontendInfo, FeCaps, FeStatus, PropertyCommands, get_info, get_properties_raw,
    read_status,
};

//
// ----- Frontend

pub struct Frontend {
    fd: OwnedFd,
    write: bool,
    info: Info,
}

impl Frontend {
    /// Open a frontend device like ```/dev/dvb/adapterX/frontendX```.
    ///
    /// Generally, an adapter need to be opened at least once as writable before it can work as read only.
    pub fn open(frontend: &Path, writeable: bool) -> Option<Frontend> {
        let flag = if writeable {
            OFlag::O_RDWR
        } else {
            OFlag::O_RDONLY
        };
        let fd = open(frontend, flag, Mode::empty()).unwrap();
        let raw_info = get_info(fd.as_fd()).unwrap();
        let info = Info::from(raw_info);

        Some(Frontend {
            fd,
            write: false,
            info,
        })
    }

    pub fn is_writeable(&self) -> bool {
        self.write
    }

    pub fn info(&self) -> &Info {
        &self.info
    }

    pub fn status(&self) -> FeStatus {
        FeStatus::from(read_status(self.fd.as_fd()).unwrap())
    }

    pub fn properties(
        &self,
        props: &mut [(PropertyCommands, &mut Option<DtvProperty>)],
    ) -> Option<()> {
        // Build requests
        let mut memory = props
            .iter()
            .map(|(cmd, _)| {
                let mut uninit: MaybeUninit<DtvProperty> = MaybeUninit::uninit();
                unsafe {
                    let r = uninit.as_mut_ptr().as_mut().unwrap();
                    r.cmd = *cmd as u32;
                }
                uninit
            })
            .collect::<Vec<_>>();

        // Send
        let first = memory[0].as_mut_ptr();
        get_properties_raw(self.fd.as_fd(), props.len(), first)?;

        // Assume init
        props
            .iter_mut()
            .zip(memory)
            .for_each(|((_, r), m)| **r = Some(unsafe { m.assume_init() }));

        Some(())
    }
}

//
// ----- Data

#[derive(Debug, Clone)]
pub struct Info {
    /// "Name of the frontend"
    pub name: String,
    pub frequency: FrequencyInfo,
    pub symbol_rate: SymbolRateInfo,
    /// "Capabilities supported by the frontend, as specified in &enum fe_caps."
    pub capabilities: FeCaps,
}

/// Frequency characteristics for this frontend.
///
/// "The frequencies are specified in Hz for Terrestrial and Cable systems."
///
/// "The frequencies are specified in kHz for Satellite systems."
#[derive(Debug, Copy, Clone)]
pub struct FrequencyInfo {
    /// Frequency range supported by the frontend.
    pub frequency_range: ValueBounds,
    /// "All frequencies are multiple of this value."
    pub frequency_step_size: u32,
    /// "Frequency tolerance."
    pub frequency_tolerance: u32,
}

/// Information related to Cable and Satellite systems.
#[derive(Debug, Copy, Clone)]
pub struct SymbolRateInfo {
    /// Symbol rate (in Bauds) supported by the frontend.
    pub symbol_rate_range: ValueBounds,
    /// "Maximal symbol rate tolerance, in ppm."
    pub symbol_rate_tolerance: u32,
}

impl From<DvbFrontendInfo> for Info {
    fn from(value: DvbFrontendInfo) -> Self {
        // TODO: This probably breaks if there is a name of size 128 bytes
        let str_ptr = &value.name as *const c_char;
        let c_str = unsafe { CStr::from_ptr(str_ptr) };
        let name = c_str.to_string_lossy().into_owned();

        Self {
            name,
            frequency: FrequencyInfo {
                frequency_range: ValueBounds::new(value.symbol_rate_min, value.symbol_rate_max),
                frequency_step_size: value.frequency_stepsize,
                frequency_tolerance: value.frequency_tolerance,
            },
            symbol_rate: SymbolRateInfo {
                symbol_rate_range: ValueBounds::new(value.symbol_rate_min, value.symbol_rate_max),
                symbol_rate_tolerance: value.symbol_rate_tolerance,
            },
            capabilities: value.caps,
        }
    }
}

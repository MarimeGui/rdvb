pub mod properties;
pub mod sys;

use std::{
    collections::HashSet,
    ffi::{CStr, c_char},
    fs::File,
    mem::MaybeUninit,
    os::fd::AsFd,
    path::Path,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    frontend::{
        properties::{
            get::{EnumerateDeliverySystems, PropertyQuery},
            set::{BandwidthHz, DeliverySystem, Frequency, SetPropertyQuery, Tune},
        },
        sys::FeDeliverySystem,
    },
    utils::ValueBounds,
};
use properties::get::QueryDescription;
use sys::{
    DvbFrontendInfo, FeCaps, FeStatus,
    ioctl::{get_info, get_set_properties_raw, read_status},
    property::DtvProperty,
};

//
// ----- Frontend

pub struct Frontend {
    file: File,
    write: bool,
    info: Info,
}

impl Frontend {
    /// Open a frontend device like ```/dev/dvb/adapterX/frontendX```.
    ///
    /// Generally, making the frontend writeable is desirable, as all the tuning operations are locked otherwise.
    /// Read-only may be useful to get the status and query statistics.
    pub fn open(path: &Path, writeable: bool) -> Option<Frontend> {
        let file = File::options()
            .read(true)
            .write(writeable)
            .open(path)
            .unwrap();
        let raw_info = get_info(file.as_fd()).unwrap();
        let info = Info::from(raw_info);

        Some(Frontend {
            file,
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

    // TODO: Should status require mutability ?
    /// Retrieve the status of the frontend.
    ///
    /// If this fails while the frontend isn't tuned, this may mean that the system is missing a required firmware.
    /// Check `dmesg` if that is the case.
    pub fn status(&self) -> FeStatus {
        FeStatus::from(read_status(self.file.as_fd()).unwrap())
    }

    pub fn properties(&mut self, props: &mut [QueryDescription]) -> Option<()> {
        // Build requests
        let mut memory = props
            .iter()
            .map(|desc| {
                let mut uninit: MaybeUninit<DtvProperty> = MaybeUninit::uninit();
                unsafe {
                    let r = uninit.as_mut_ptr().as_mut().unwrap();
                    r.cmd = desc.command as u32;
                }
                uninit
            })
            .collect::<Vec<_>>();

        // Send
        let first = memory[0].as_mut_ptr();
        get_set_properties_raw(self.file.as_fd(), false, props.len(), first)?;

        // Assume init
        props
            .iter_mut()
            .zip(memory)
            .for_each(|(desc, m)| *desc.property = Some(unsafe { m.assume_init() }));

        Some(())
    }

    // For now, it is convenient to just have a slice of DtvProperty as it already is setup in memory correctly for IOCTL
    // TODO: That should require &mut self, look into File to see how they do it
    pub fn set_properties(&mut self, props: &mut [DtvProperty]) -> Option<()> {
        get_set_properties_raw(self.file.as_fd(), true, props.len(), props.as_mut_ptr())?;
        Some(())
    }

    /// Tunes the frontend for a given system, bandwidth and frequency.
    ///
    /// This is equivalent to using [`set_properties`](Self::set_properties) with [`Frequency`], [`DeliverySystem`], [`BandwidthHz`] and [`Tune`] properties.
    /// This function is here for convenience.
    pub fn tune(
        &mut self,
        frequency: u32,
        delivery_system: FeDeliverySystem,
        bandwidth: BandwidthHz,
    ) -> Option<()> {
        let freq = Frequency::new(frequency);
        let del_sys = DeliverySystem::new(delivery_system);
        let tune = Tune {};
        self.set_properties(&mut [
            freq.property(),
            bandwidth.property(),
            del_sys.property(),
            tune.property(),
        ])
    }

    /// Blocks execution until the tuned frontend has a lock on a transponder.
    ///
    /// Returns `true` if the frontend locked in successfully, `false` otherwise.
    pub fn wait_for_lock(
        &self,
        timeout: Option<Duration>,
        poll_interval: Option<Duration>,
    ) -> bool {
        let poll_interval = poll_interval.unwrap_or(Duration::from_millis(100));

        let start_time = Instant::now();
        loop {
            // Check if locked
            if self.status().has_lock() {
                return true;
            }
            if let Some(timeout) = timeout {
                // Timeout
                if (Instant::now() - start_time) > timeout {
                    return false;
                }
            }
            sleep(poll_interval);
        }
    }

    /// Return a list of all delivery systems (DVB-T, DVB-T2, SVB-S...) this frontend supports.
    ///
    /// This is equivalent to using `properties` with `EnumerateDeliverySystems` property query. This function is for convenience.
    pub fn list_systems(&mut self) -> Option<HashSet<FeDeliverySystem>> {
        let mut enumerate_systems = EnumerateDeliverySystems::query();
        self.properties(&mut [enumerate_systems.desc()]);
        Some(enumerate_systems.retrieve().unwrap().0)
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

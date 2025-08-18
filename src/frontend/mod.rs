pub mod properties;
pub mod sys;

use std::{
    collections::BTreeSet,
    ffi::{CStr, c_char},
    fs::File,
    mem::MaybeUninit,
    os::fd::AsFd,
    path::Path,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    error::FrontendError,
    frontend::{
        properties::{
            get::{EnumerateDeliverySystems, PropertyQuery, SignalStrength},
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

type Result<T> = std::result::Result<T, FrontendError>;

impl Frontend {
    /// Open a frontend device like ```/dev/dvb/adapterX/frontendX```.
    ///
    /// Generally, making the frontend writeable is desirable, as all the tuning operations are unavailable otherwise.
    /// Read-only may be useful to get the status and query statistics.
    pub fn open(path: &Path, writeable: bool) -> Result<Frontend> {
        let file = File::options()
            .read(true)
            .write(writeable)
            .open(path)
            .map_err(FrontendError::Open)?;
        let raw_info = get_info(file.as_fd()).map_err(FrontendError::InfoQuery)?;
        let info = Info::from(raw_info);

        Ok(Frontend {
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
    pub fn status(&self) -> Result<FeStatus> {
        Ok(FeStatus::from(
            read_status(self.file.as_fd()).map_err(FrontendError::StatusQuery)?,
        ))
    }

    pub fn properties(&mut self, props: &mut [QueryDescription]) -> Result<()> {
        // Build requests
        let mut memory = props
            .iter()
            .map(|desc| {
                let mut uninit: MaybeUninit<DtvProperty> = MaybeUninit::uninit();
                // SAFETY: Only `cmd` field is written to, no other field is read.
                unsafe {
                    let r = uninit.as_mut_ptr().as_mut().unwrap();
                    r.cmd = desc.command as u32;
                }
                uninit
            })
            .collect::<Vec<_>>();

        // Send
        let first = memory[0].as_mut_ptr();
        get_set_properties_raw(self.file.as_fd(), false, props.len(), first)
            .map_err(FrontendError::Property)?;

        // Assume init
        // SAFETY: As long as get_set_properties_raw did not throw an error, all memory should have been filled out.
        props
            .iter_mut()
            .zip(memory)
            .for_each(|(desc, m)| *desc.property = Some(unsafe { m.assume_init() }));

        Ok(())
    }

    // For now, it is convenient to just have a slice of DtvProperty as it already is setup in memory correctly for IOCTL
    // TODO: That should require &mut self, look into File to see how they do it
    pub fn set_properties(&mut self, props: &mut [DtvProperty]) -> Result<()> {
        get_set_properties_raw(self.file.as_fd(), true, props.len(), props.as_mut_ptr())
            .map_err(FrontendError::Property)?;
        Ok(())
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
    ) -> Result<()> {
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
    ) -> Result<bool> {
        let poll_interval = poll_interval.unwrap_or(Duration::from_millis(50));

        let start_time = Instant::now();
        loop {
            // Check if locked
            if self.status()?.has_lock() {
                return Ok(true);
            }
            if let Some(timeout) = timeout {
                // Timeout
                if (Instant::now() - start_time) > timeout {
                    return Ok(false);
                }
            }
            sleep(poll_interval);
        }
    }

    /// Return a list of all delivery systems (DVB-T, DVB-T2, SVB-S...) this frontend supports.
    ///
    /// This is equivalent to using `properties` with `EnumerateDeliverySystems` property query. This function is for convenience.
    pub fn list_systems(&mut self) -> Result<BTreeSet<FeDeliverySystem>> {
        let mut enumerate_systems = EnumerateDeliverySystems::query();
        self.properties(&mut [enumerate_systems.desc()])?;
        Ok(enumerate_systems
            .retrieve()
            .map_err(FrontendError::Retrieve)?
            .0)
    }

    /// Get a reading of the strength of the signal being received.
    ///
    /// This may be useful to compare two different frequencies over which the same transponder is received and choose the best one.
    pub fn signal_strength(&mut self) -> Result<SignalStrength> {
        let mut strength = SignalStrength::query();
        self.properties(&mut [strength.desc()])?;
        strength.retrieve().map_err(FrontendError::Retrieve)
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

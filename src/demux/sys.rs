use std::{
    ffi::c_uint,
    os::fd::{AsRawFd, BorrowedFd},
};

use nix::{errno::Errno, ioctl_none, ioctl_read, ioctl_readwrite, ioctl_write_ptr};

use crate::{
    IOCTL_TYPE,
    error::{DmxSetPesFilterError, DmxStartError},
};

//
// ----- IOCTLs

const DMX_START: u8 = 41;
ioctl_none!(dmx_start, IOCTL_TYPE, DMX_START);

const DMX_STOP: u8 = 42;
ioctl_none!(dmx_stop, IOCTL_TYPE, DMX_STOP);

const DMX_SET_FILTER: u8 = 43;
ioctl_write_ptr!(
    dmx_set_filter,
    IOCTL_TYPE,
    DMX_SET_FILTER,
    DmxSctFilterParams
);

const DMX_SET_PES_FILTER: u8 = 44;
ioctl_write_ptr!(
    dmx_set_pes_filter,
    IOCTL_TYPE,
    DMX_SET_PES_FILTER,
    DmxPesFilterParams
);

const DMX_SET_BUFFER_SIZE: u8 = 45;
// TODO: dmx.h and documentation are inconsistent, header says there is no parameter while docs want an unsigned long for size
ioctl_none!(dmx_set_buffer_size, IOCTL_TYPE, DMX_SET_BUFFER_SIZE);

const DMX_GET_PES_PIDS: u8 = 47;
ioctl_read!(dmx_get_pes_pids, IOCTL_TYPE, DMX_GET_PES_PIDS, [u16; 5]);

const DMX_GET_STC: u8 = 50;
ioctl_readwrite!(dmx_get_stc, IOCTL_TYPE, DMX_GET_STC, DmxStc);

const DMX_ADD_PID: u8 = 51;
ioctl_write_ptr!(dmx_add_pid, IOCTL_TYPE, DMX_ADD_PID, u16);

const DMX_REMOVE_PID: u8 = 52;
ioctl_write_ptr!(dmx_remove_pid, IOCTL_TYPE, DMX_REMOVE_PID, u16);

// TODO: Experimental IOCTLs

//
// ----- Simplified IOCTLs

// TODO: This really isn't "sys" anymore... If I wanted to move all sys to a separate crate, these probably won't be included...
// Should probably directly put this code in the struct impl...

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx-start.html#description))
///
/// This ioctl call is used to start the actual filtering operation defined via the ioctl calls DMX_SET_FILTER or DMX_SET_PES_FILTER.
pub fn start(fd: BorrowedFd) -> Result<(), DmxStartError> {
    unsafe { dmx_start(fd.as_raw_fd()) }.map_err(DmxStartError::from)?;
    Ok(())
}

pub fn stop(fd: BorrowedFd) -> Result<(), Errno> {
    unsafe { dmx_stop(fd.as_raw_fd()) }?;
    Ok(())
}

pub fn set_filter(fd: BorrowedFd, params: &DmxSctFilterParams) -> Result<(), Errno> {
    unsafe { dmx_set_filter(fd.as_raw_fd(), params) }?;
    Ok(())
}

pub fn set_pes_filter(
    fd: BorrowedFd,
    params: &DmxPesFilterParams,
) -> Result<(), DmxSetPesFilterError> {
    unsafe { dmx_set_pes_filter(fd.as_raw_fd(), params) }.map_err(DmxSetPesFilterError::from)?;
    Ok(())
}

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx-add-pid.html#description))
///
/// This ioctl call allows to add multiple PIDs to a transport stream filter previously
/// set up with DMX_SET_PES_FILTER and output equal to DMX_OUT_TSDEMUX_TAP.
pub fn add_pid(fd: BorrowedFd, pid: u16) -> Result<(), Errno> {
    unsafe { dmx_add_pid(fd.as_raw_fd(), &pid) }?;
    Ok(())
}

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx-remove-pid.html#description))
///
/// This ioctl call allows to remove a PID when multiple PIDs are set on a transport stream filter,
/// e. g. a filter previously set up with output equal to DMX_OUT_TSDEMUX_TAP,
/// created via either DMX_SET_PES_FILTER or DMX_ADD_PID.
pub fn remove_pid(fd: BorrowedFd, pid: u16) -> Result<(), Errno> {
    unsafe { dmx_remove_pid(fd.as_raw_fd(), &pid) }?;
    Ok(())
}

//
// ----- Data

pub const DMX_FILTER_SIZE: usize = 16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum DmxOutput {
    DMX_OUT_DECODER,
    DMX_OUT_TAP,
    DMX_OUT_TS_TAP,
    DMX_OUT_TSDEMUX_TAP,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum DmxInput {
    DMX_IN_FRONTEND,
    DMX_IN_DVR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum DmxTsPes {
    DMX_PES_AUDIO0,
    DMX_PES_VIDEO0,
    DMX_PES_TELETEXT0,
    DMX_PES_SUBTITLE0,
    DMX_PES_PCR0,

    DMX_PES_AUDIO1,
    DMX_PES_VIDEO1,
    DMX_PES_TELETEXT1,
    DMX_PES_SUBTITLE1,
    DMX_PES_PCR1,

    DMX_PES_AUDIO2,
    DMX_PES_VIDEO2,
    DMX_PES_TELETEXT2,
    DMX_PES_SUBTITLE2,
    DMX_PES_PCR2,

    DMX_PES_AUDIO3,
    DMX_PES_VIDEO3,
    DMX_PES_TELETEXT3,
    DMX_PES_SUBTITLE3,
    DMX_PES_PCR3,

    DMX_PES_OTHER,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxFilter {
    pub filter: [u8; DMX_FILTER_SIZE],
    pub mask: [u8; DMX_FILTER_SIZE],
    pub mode: [u8; DMX_FILTER_SIZE],
}

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx_types.html#c.dmx_sct_filter_params))
///
/// Specifies a section filter.
///
/// Carries the configuration for a MPEG-TS section filter.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxSctFilterParams {
    /// PID to be filtered.
    pub pid: u16,
    /// section header filter, as defined by struct dmx_filter.
    pub filter: DmxFilter,
    /// maximum time to filter, in milliseconds.
    pub timeout: u32,
    // TODO: u32 struct for bits
    /// extra flags for the section filter.
    pub flags: u32,
}

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx_types.html#c.dmx_pes_filter_params))
///
/// Specifies Packetized Elementary Stream (PES) filter parameters.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxPesFilterParams {
    /// PID to be filtered.
    pub pid: u16,
    /// Demux input, as specified by enum dmx_input.
    pub input: DmxInput,
    /// Demux output, as specified by enum dmx_output.
    pub output: DmxOutput,
    /// Type of the pes filter, as specified by enum dmx_pes_type.
    pub pes_type: DmxTsPes,
    // TODO: There is an enum for these flags
    /// Demux PES flags.
    pub flags: u32,
}

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx_types.html#c.dmx_stc))
///
/// Stores System Time Counter (STC) information.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxStc {
    /// input data: number of the STC, from 0 to N.
    pub num: c_uint,
    /// output: divisor for STC to get 90 kHz clock.
    pub base: c_uint,
    /// output: stc in **base** * 90 kHz units.
    pub stc: u64,
}

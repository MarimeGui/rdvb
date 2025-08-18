use std::{
    ffi::c_uint,
    mem::MaybeUninit,
    os::fd::{AsRawFd, BorrowedFd},
};

use nix::{errno::Errno, ioctl_read, ioctl_write_ptr};

use crate::error::PropertyError;

use super::{
    DTV_IOCTL_MAX_MSGS, DvbFrontendInfo,
    property::{DtvProperties, DtvProperty},
};

//
// ----- IOCTLs

const FE_TYPE: u8 = b'o';

const FE_GET_INFO: u8 = 61;
ioctl_read!(fe_get_info, FE_TYPE, FE_GET_INFO, DvbFrontendInfo);

const FE_READ_STATUS: u8 = 69;
ioctl_read!(fe_read_status, FE_TYPE, FE_READ_STATUS, c_uint); // Maps to FeStatus struct for bits

const FE_SET_PROPERTY: u8 = 82;
ioctl_write_ptr!(fe_set_property, FE_TYPE, FE_SET_PROPERTY, DtvProperties);

const FE_GET_PROPERTY: u8 = 83;
ioctl_read!(fe_get_property, FE_TYPE, FE_GET_PROPERTY, DtvProperties);

//
// ----- Simplified IOCTLs

pub fn get_info(fd: BorrowedFd) -> Result<DvbFrontendInfo, Errno> {
    let mut info = MaybeUninit::uninit();
    unsafe { fe_get_info(fd.as_raw_fd(), info.as_mut_ptr()) }?;
    // SAFETY: If fe_get_info did not throw an error, memory should now be initialized.
    let info = unsafe { info.assume_init() };
    Ok(info)
}

pub fn read_status(fd: BorrowedFd) -> Result<c_uint, Errno> {
    let mut status = MaybeUninit::uninit();
    unsafe { fe_read_status(fd.as_raw_fd(), status.as_mut_ptr()) }?;
    // SAFETY: If fe_read_status did not throw an error, memory should now be initialized.
    let status = unsafe { status.assume_init() };
    Ok(status)
}

pub fn get_set_properties_raw(
    fd: BorrowedFd,
    set: bool,
    count: usize,
    ptr: *mut DtvProperty,
) -> Result<(), PropertyError> {
    if count == 0 {
        return Ok(());
    }

    if count > DTV_IOCTL_MAX_MSGS {
        return Err(PropertyError::TooManyParameters);
    }

    let mut properties = DtvProperties {
        num: count as u32,
        props: ptr,
    };

    if set {
        unsafe { fe_set_property(fd.as_raw_fd(), &mut properties as *mut DtvProperties) }
            .map_err(PropertyError::SetProperty)?;
    } else {
        unsafe { fe_get_property(fd.as_raw_fd(), &mut properties as *mut DtvProperties) }
            .map_err(PropertyError::GetProperty)?;
    }

    Ok(())
}

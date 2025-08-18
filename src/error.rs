use std::{ffi::c_int, num::ParseIntError};

use nix::errno::Errno;
use thiserror::Error;

//
// -----

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx-start.html#return-value))
#[derive(Error, Debug)]
pub enum DmxStartError {
    /// Invalid argument, i.e. no filtering parameters provided via the DMX_SET_FILTER or DMX_SET_PES_FILTER ioctls.
    #[error("invalid arguments for filter")]
    InvalidArgument,
    /// This error code indicates that there are conflicting requests. There are active filters filtering data from another input source. Make sure that these filters are stopped before starting this filter.
    #[error("already filtering from another input source")]
    Conflicting,
    #[error("undefined error from ioctl")]
    Undefined(Errno),
}

impl From<Errno> for DmxStartError {
    fn from(value: Errno) -> Self {
        match value {
            Errno::EINVAL => DmxStartError::InvalidArgument,
            Errno::EBUSY => DmxStartError::Conflicting,
            e => DmxStartError::Undefined(e),
        }
    }
}

//
// -----

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx-set-pes-filter.html#return-value))
#[derive(Error, Debug)]
pub enum DmxSetPesFilterError {
    #[error("already filtering from another input source")]
    Conflicting,
    #[error("undefined error from ioctl")]
    Undefined(Errno),
}

impl From<Errno> for DmxSetPesFilterError {
    fn from(value: Errno) -> Self {
        match value {
            Errno::EBUSY => DmxSetPesFilterError::Conflicting,
            e => DmxSetPesFilterError::Undefined(e),
        }
    }
}

//
// -----

#[derive(Error, Debug)]
pub enum VdrParseError {
    #[error("the channel definition line is missing at least 1 column")]
    MissingColumn,
    #[error("expected an int for field contents")]
    IntParse(ParseIntError),
    #[error("a value outside of accepted variants was found as parameter data")]
    UnexpectedParameterValue,
    #[error("an unknown parameter was found")]
    UnknownParameter,
}

//
// -----

#[derive(Error, Debug)]
pub enum FrontendError {
    #[error("problem while opening frontend")]
    Open(std::io::Error),
    #[error("failed to query information about frontend")]
    InfoQuery(Errno),
    #[error("failed to query current status of frontend")]
    StatusQuery(Errno),
    #[error("problem while using properties")]
    Property(PropertyError),
    #[error("results of a query indicate an error")]
    Retrieve(DtvError),
}

//
// -----

#[derive(Error, Debug)]
pub enum PropertyError {
    #[error("requested too many parameters at once")]
    TooManyParameters,
    #[error("problem while reading one or more properties")]
    GetProperty(Errno),
    #[error("problem while writing one or more properties")]
    SetProperty(Errno),
}

//
// -----

#[derive(Error, Debug)]
pub enum DtvError {
    #[error("tried to receive information from a query that wasn't ran")]
    NotRan,
    #[error("kernel application returned an error")]
    Reported(c_int),
}

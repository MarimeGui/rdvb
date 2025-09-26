use std::{ffi::c_int, num::ParseIntError};

use nix::errno::Errno;
use rdvb_os_linux::error::PropertyError;
use thiserror::Error;

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
pub enum DtvError {
    #[error("tried to receive information from a query that wasn't ran")]
    NotRan,
    #[error("kernel application returned an error")]
    Reported(c_int),
}

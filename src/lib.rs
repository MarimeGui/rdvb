//! The goal here is to have the exact same API provided by the [kernel APIs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dvbapi.html) but in a more rusty way.

pub mod demux;
pub mod frontend;
pub mod utils;

/// For all IOCTLs related to DVB
pub(crate) const IOCTL_TYPE: u8 = b'o';

use nix::ioctl_none;

use crate::IOCTL_TYPE;

//
// ----- IOCTLs

const DMX_START: u8 = 41;
ioctl_none!(dmx_start, IOCTL_TYPE, DMX_START);

const DMX_STOP: u8 = 42;
ioctl_none!(dmx_stop, IOCTL_TYPE, DMX_STOP);

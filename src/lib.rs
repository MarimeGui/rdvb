//! # `rdvb`
//! Welcome to `rdvb` ! This crate allows Linux/Rust users to manipulate DVB devices for watching TV and more.
//!
//! This is my *interpretation* of pieces of information I could gather. This is because :
//! 1. The [DVBv5 docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dvbapi.html) are terrible,
//! 2. The Linux kernel code is not super straightforward,
//! 3. All DVB-related docs are not publicly available or behind a paywall,
//! 4. The official docs assumes one already understands the whole system while reading the document,
//! 5. The official docs are laid-out in a way that separates all the info that would have been useful together
//! 6. DVB is an old, over-encompassing standard. (do we *seriously* need a dedicated stream type for H.264-frame-compliant Plano-Stereoscopic High Definition Video-on-Demand Time Shifting ??)
//!
//! The goal here is to provide APIs for most DVB use-cases in pure Rust.
//!
//! You should probably start by opening a [Frontend](frontend::Frontend) and then create a [Demux](demux::Demux) to filter out some Packets from the multiplex.
//!
//! # How does DVB work as a whole ?
//! This isn't well documented anywhere, so I guess I'll write what I understand of the system and how our USB sticks work with the kernel to get us to a TV show in a software player.
//!
//! A local TV-provider has an antenna somewhere around the user and beams out DVB-T.
//! The user picks it up with an antenna connected to their USB receiver.
//!
//! From there, the Frontend (as in radio frequency frontend) of the receiver directly connects to the antenna.
//! The user, through the Linux kernel IOCTLs, tells the frontend to tune for a particular [delivery system](frontend::sys::FeDeliverySystem) (DVB-T, DVB-T2, DVB-S...) with a specific bandwidth and frequency.
//! The frontend should then handle all of the radio-frequency business.
//!
//! If everything works out, the frontend should enter a locked state ([`has_lock`](frontend::sys::FeStatus::has_lock)) and start streaming the whole multiplex to the demuxer.
//! A multiplex is a single MPEG Transport Stream (TS) containing multiple TV channels, including multiple audio and video streams, along with other data.
//!
//! As far as I know, there is no direct way to access that raw multiplex from software (I may be wrong here).
//!
//! Instead, the user now opens a file descriptor to the demux file of the adapter.
//! From there, they can set a filter that will selectively choose specific packets to send over to the program.
//!

pub mod bands;
pub mod conf;
pub mod demux;
pub mod error;
pub mod frontend;
pub mod utils;

/// For all IOCTLs related to DVB
pub(crate) const IOCTL_TYPE: u8 = b'o';

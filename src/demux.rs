use std::{fs::File, io::Read, os::fd::AsFd, path::Path, time::Duration};

use rdvb_os_linux::demux::{
    data::{DmxFilter, DmxSctFilterParams},
    functions::{set_filter, start, stop},
};

use crate::mpeg::{DMX_CHECK_CRC, DMX_IMMEDIATE_START, DMX_ONESHOT, Packet};

pub struct Demux {
    file: File,
}

impl Demux {
    pub fn new(demux: &Path) -> Result<Demux, std::io::Error> {
        let file = File::open(demux)?;
        Ok(Demux { file })
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.file.read(buf)
    }

    pub fn start(&mut self) {
        start(self.file.as_fd()).unwrap()
    }

    pub fn stop(&mut self) {
        stop(self.file.as_fd()).unwrap()
    }

    /// Setup a general filter to let some packets through.
    pub fn set_filter(&mut self, filter: &DmxSctFilterParams) {
        set_filter(self.file.as_fd(), filter).unwrap()
    }

    /// Setup this instance to only filter a single valid packet with provided PID and optional Table ID, starting immediately.
    pub fn filter_one(&mut self, pid: u16, table_id: Option<u8>, timeout: Option<Duration>) {
        // Table ID is always the first byte for SI packets.
        // Therefore, add a filter that checks this first byte against provided table_id.
        let mut inner_filter = DmxFilter::default();
        if let Some(table_id) = table_id {
            inner_filter.first_byte_mask(table_id);
        }

        let filter = DmxSctFilterParams {
            pid,
            filter: inner_filter,
            timeout: timeout.map(|d| d.as_millis() as u32).unwrap_or(0),
            flags: DMX_CHECK_CRC | DMX_ONESHOT | DMX_IMMEDIATE_START, // TODO: Proper thing later
        };

        self.set_filter(&filter);
    }

    /// Receive a single data packet from the interface. This implies a properly set-up filter.
    pub fn read_one_packet(&mut self) -> Result<Packet, std::io::Error> {
        let mut buf = vec![0; 4096];
        let read = self.read(&mut buf)?;
        buf.truncate(read);
        Ok(Packet::from_buf(&buf))
    }
}

// TODO: Get one packet with trait for specific section ?

pub struct PidTableIdPair {
    pub pid: u16,
    pub table_id: Option<u8>,
}

/// Receives a single packet for each specified PID and optional Table ID.
pub fn receive_multiple_single_packets(
    demux_path: &Path,
    pairs: Vec<PidTableIdPair>,
    timeout: Option<Duration>,
) -> Result<Vec<Packet>, std::io::Error> {
    // First, setup all demuxers for all requested pairs
    let mut demuxers = Vec::new();
    for pair in pairs {
        let mut demux = Demux::new(demux_path)?;
        demux.filter_one(pair.pid, pair.table_id, timeout);
        demuxers.push(demux);
    }

    // Now, the kernel will keep a single packet as it arrives, and we can block on reading all of them

    // Read all demuxers
    let mut packets = Vec::new();
    for mut demux in demuxers.into_iter() {
        packets.push(demux.read_one_packet()?);
    }
    Ok(packets)
}

/// Receives a single packet for a PID and optional table ID.
pub fn receive_single_packet(
    demux_path: &Path,
    pid: u16,
    table_id: Option<u8>,
    timeout: Option<Duration>,
) -> Result<Packet, std::io::Error> {
    let packets = receive_multiple_single_packets(
        demux_path,
        vec![PidTableIdPair { pid, table_id }],
        timeout,
    )?;
    let p = packets.into_iter().next().unwrap();
    Ok(p)
}

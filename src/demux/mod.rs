pub mod sys;

use std::{fs::File, io::Read, os::fd::AsFd, path::Path};

use sys::{DmxSctFilterParams, set_filter, start, stop};

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

    pub fn start(&self) {
        start(self.file.as_fd()).unwrap()
    }

    pub fn stop(&self) {
        stop(self.file.as_fd()).unwrap()
    }

    pub fn set_filter(&self, filter: &DmxSctFilterParams) {
        set_filter(self.file.as_fd(), filter).unwrap()
    }
}

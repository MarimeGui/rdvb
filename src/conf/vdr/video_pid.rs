use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
pub struct VideoPID {
    pub pcr_pid: u16,
    pub video_pid: Option<u16>,
    /// Not shown when 0
    pub video_mode: u16,
}

impl FromStr for VideoPID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once('+') {
            Some((vpid, rest)) => {
                match rest.split_once('=') {
                    // Separated Video PID, PCR PID and Video mode (like "164+17=27")
                    Some((pcr_pid, video_mode)) => VideoPID {
                        pcr_pid: pcr_pid.parse()?,
                        video_pid: Some(vpid.parse()?),
                        video_mode: video_mode.parse()?,
                    },
                    // Separate Video PID and PCR PID (like "164+17")
                    None => VideoPID {
                        pcr_pid: vpid.parse()?,
                        video_pid: Some(rest.parse()?),
                        video_mode: 0,
                    },
                }
            }
            None => {
                match s.split_once('=') {
                    // Separated PCR PID and Video mode (like "164=27")
                    Some((pcr_pid, video_mode)) => VideoPID {
                        pcr_pid: pcr_pid.parse()?,
                        video_pid: None,
                        video_mode: video_mode.parse()?,
                    },
                    // Only PCR PID (like "164")
                    None => VideoPID {
                        pcr_pid: s.parse()?,
                        video_pid: None,
                        video_mode: 0,
                    },
                }
            }
        })
    }
}

impl VideoPID {
    pub fn format(&self) -> String {
        match (self.video_pid, self.video_mode != 0) {
            (None, false) => self.pcr_pid.to_string(),
            (Some(video_pid), false) => format!("{}+{}", video_pid, self.pcr_pid),
            (None, true) => format!("{}={}", self.pcr_pid, self.video_mode),
            (Some(video_pid), true) => {
                format!("{}+{}={}", video_pid, self.pcr_pid, self.video_mode)
            }
        }
    }
}

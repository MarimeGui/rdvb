use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Default)]
pub struct TeletextPIDList {
    pub teletext: Vec<u16>,
    pub subtitles: Vec<SubtitlePID>,
}

#[derive(Debug, Clone)]
pub struct SubtitlePID {
    pub pid: u16,
    pub language: String,
}

impl TeletextPIDList {
    fn parse_teletext(s: &str) -> Result<Vec<u16>, ParseIntError> {
        let mut teletext = Vec::new();
        for p in s.split(',') {
            teletext.push(p.parse()?);
        }
        Ok(teletext)
    }

    fn parse_subtitles(s: &str) -> Result<Vec<SubtitlePID>, ParseIntError> {
        let mut subtitles = Vec::new();
        for p in s.split(',') {
            subtitles.push(p.parse()?);
        }
        Ok(subtitles)
    }
}

impl FromStr for TeletextPIDList {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "0" {
            return Ok(TeletextPIDList::default());
        }

        Ok(match s.split_once(';') {
            Some((teletext, subtitles)) => {
                if teletext == "0" {
                    // Just subtitles
                    TeletextPIDList {
                        teletext: vec![],
                        subtitles: Self::parse_subtitles(subtitles)?,
                    }
                } else {
                    // Both teletext and subtitles
                    TeletextPIDList {
                        teletext: Self::parse_teletext(teletext)?,
                        subtitles: Self::parse_subtitles(subtitles)?,
                    }
                }
            }
            None => {
                // Just teletext
                TeletextPIDList {
                    teletext: Self::parse_teletext(s)?,
                    subtitles: vec![],
                }
            }
        })
    }
}

impl FromStr for SubtitlePID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once('=') {
            Some((pid, lang)) => SubtitlePID {
                pid: pid.parse()?,
                language: lang.to_string(),
            },
            None => SubtitlePID {
                pid: s.parse()?,
                language: String::new(),
            },
        })
    }
}

impl TeletextPIDList {
    pub fn format(&self) -> String {
        if self.subtitles.is_empty() & self.teletext.is_empty() {
            return "0".to_string();
        }

        todo!()
    }
}

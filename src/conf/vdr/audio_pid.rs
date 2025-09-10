use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Default)]
pub struct AudioPIDList {
    pub regular_pids: Vec<AudioPID>,
    pub dolby_pids: Vec<AudioPID>,
}

#[derive(Debug, Clone)]
pub struct AudioPID {
    pub pid: u16,
    pub language_code: String,
    pub second_language_code: String,
    pub audio_type: Option<u16>,
}

impl AudioPIDList {
    fn parse_part(s: &str) -> Result<Vec<AudioPID>, ParseIntError> {
        let mut entries = Vec::new();
        for one_pid in s.split(',') {
            entries.push(one_pid.parse()?);
        }
        Ok(entries)
    }
}

impl FromStr for AudioPIDList {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(';') {
            Some((regular, dolby)) => {
                if regular == "0" {
                    AudioPIDList {
                        regular_pids: vec![],
                        dolby_pids: Self::parse_part(dolby)?,
                    }
                } else {
                    AudioPIDList {
                        regular_pids: Self::parse_part(regular)?,
                        dolby_pids: Self::parse_part(dolby)?,
                    }
                }
            }
            None => AudioPIDList {
                regular_pids: Self::parse_part(s)?,
                dolby_pids: vec![],
            },
        })
    }
}

impl FromStr for AudioPID {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try splitting off the audio type first
        let (rest, audio_type) = match s.split_once('@') {
            Some((rest, audio_type)) => (rest, Some(audio_type.parse()?)),
            None => (s, None),
        };

        // Try splitting the language
        let (pid, languages) = match rest.split_once('=') {
            Some((pid, langauges)) => (pid.parse()?, Some(langauges)),
            None => (rest.parse()?, None),
        };

        // Finally, if there is a language string, try splitting it
        let (language_code, second_language_code) = match languages {
            Some(l) => match l.split_once('+') {
                Some((first, second)) => (first.to_string(), second.to_string()),
                None => (l.to_string(), String::new()),
            },
            None => (String::new(), String::new()),
        };

        Ok(AudioPID {
            pid,
            language_code,
            second_language_code,
            audio_type,
        })
    }
}

impl AudioPIDList {
    pub fn format(&self) -> String {
        let mut list = String::new();

        if self.regular_pids.is_empty() {
            list.push('0')
        } else {
            let mut list = String::new();
            let mut first = true;
            for pid in &self.regular_pids {
                if first {
                    first = false;
                } else {
                    list.push(',');
                }
                list.push_str(&pid.format());
            }
        }

        if !self.dolby_pids.is_empty() {
            list.push(';');
            let mut first = true;
            for pid in &self.dolby_pids {
                if first {
                    first = false;
                } else {
                    list.push(',');
                }
                list.push_str(&pid.format());
            }
        }

        list
    }
}

impl AudioPID {
    pub fn format(&self) -> String {
        match (
            !self.language_code.is_empty(),
            !self.second_language_code.is_empty(),
            self.audio_type,
        ) {
            (false, false, None) => self.pid.to_string(),
            (false, false, Some(audio_type)) => format!("{}@{}", self.pid, audio_type),
            (true, false, None) => {
                format!("{}={}", self.pid, self.language_code)
            }
            (true, false, Some(audio_type)) => {
                format!("{}={}@{}", self.pid, self.language_code, audio_type)
            }
            _ => todo!(),
        }
    }
}

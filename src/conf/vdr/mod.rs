//! VDR-style configuration import/export

pub mod audio_pid;
pub mod parameters;
pub mod teletext_pid;
pub mod video_pid;

use std::str::FromStr;

use crate::error::VdrParseError;
use audio_pid::AudioPIDList;
use parameters::Parameters;
use teletext_pid::TeletextPIDList;
use video_pid::VideoPID;

//
// -----

/// Parse an entire VDR file.
pub fn from_list_str(s: &str) -> Vec<ChannelDefinition> {
    let mut channels = Vec::new();
    for line in s.lines() {
        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Skip comments
        if line.starts_with('#') {
            continue;
        }

        // Groups and channel numbers
        if line.starts_with(':') {
            // TODO: Parse
            continue;
        }

        let channel = ChannelDefinition::from_str(line).unwrap();
        channels.push(channel);
    }

    channels
}

//
// -----

/// A single line of a VDR-style configuration file.
///
/// Example taken from [`man`](https://manpages.ubuntu.com/manpages/xenial/man5/vdr.5.html):
/// ```RTL Television,RTL;RTL World:12187:hC34M2O0S0:S19.2E:27500:163=2:104=deu;106=deu:105:0:12003:1:1089:0```
#[derive(Debug, Clone)]
pub struct ChannelDefinition {
    pub name: String,
    pub short_name: String,
    pub bouquet: String,
    pub frequency: u32,
    pub parameters: Parameters,
    // Always 'T' for DVB-T and T2
    pub source: String,
    pub symbol_rate: u32,
    pub video_pid: VideoPID,
    pub audio_pid: AudioPIDList,
    pub teletext_pid: TeletextPIDList,
    // `0` for free-to-air
    pub conditional_access: String,
    // program_number in PMT, found in NIT
    pub service_id: u16,
    // Found in NIT
    pub network_id: u16,
    // Found in NIT
    pub transport_stream_id: u16,
    // `0` for television
    pub radio_id: u16,
}

impl FromStr for ChannelDefinition {
    type Err = VdrParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut iter = line.split(':');

        let names = iter.next().ok_or(VdrParseError::MissingColumn)?;
        let frequency = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let parameters = iter.next().ok_or(VdrParseError::MissingColumn)?.parse()?;
        let source = iter.next().ok_or(VdrParseError::MissingColumn)?.to_string();
        let symbol_rate = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .to_string()
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let video_pid = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let audio_pid = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let teletext_pid = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let conditional_access = iter.next().ok_or(VdrParseError::MissingColumn)?.to_string();
        let service_id = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let network_id = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let transport_stream_id = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;
        let radio_id = iter
            .next()
            .ok_or(VdrParseError::MissingColumn)?
            .parse()
            .map_err(VdrParseError::IntParse)?;

        // Separate bouquet from rest
        let (rest, bouquet) = if let Some((a, b)) = names.rsplit_once(';') {
            (a, b.to_string())
        } else {
            (names, String::new())
        };

        // Separate name from short name
        let (name, short_name) = if let Some((a, b)) = rest.rsplit_once(',') {
            (a.to_string(), b.to_string())
        } else {
            (rest.to_string(), String::new())
        };

        // Replacement characters
        let name = name.replace("|", ":");

        Ok(ChannelDefinition {
            name,
            short_name,
            bouquet,
            frequency,
            parameters,
            source,
            symbol_rate,
            video_pid,
            audio_pid,
            teletext_pid,
            conditional_access,
            service_id,
            network_id,
            transport_stream_id,
            radio_id,
        })
    }
}

impl ChannelDefinition {
    pub fn format(&self) -> String {
        // TODO: Replacement characters
        let name = match (!self.short_name.is_empty(), !self.bouquet.is_empty()) {
            (false, false) => &self.name,
            (false, true) => &format!("{};{}", self.name, self.bouquet),
            (true, false) => &format!("{},{}", self.name, self.short_name),
            (true, true) => &format!("{},{};{}", self.name, self.short_name, self.bouquet),
        };

        format!(
            "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            name,
            self.frequency,
            self.parameters.format(),
            self.source,
            self.symbol_rate,
            self.video_pid.format(),
            self.audio_pid.format(),
            self.teletext_pid.format(),
            self.conditional_access,
            self.service_id,
            self.network_id,
            self.transport_stream_id,
            self.radio_id
        )
    }
}

//
// -----

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::conf::vdr::ChannelDefinition;

    #[test]
    fn parse() {
        let example = "RTL Television,RTL;RTL World:12187:hC34M2O0S0:S19.2E:27500:163=2:104=deu;106=deu:105:0:12003:1:1089:0";

        let parsed = ChannelDefinition::from_str(example).unwrap();
    }

    // TODO: Complete this test
    // fn complex_export() {
    //     let channel = ChannelDefinition {
    //         name: "rdvb: Some channel !",
    //         short_name: "rdvb",
    //         bouquet: "Rust",
    //         frequency: todo!(),
    //         parameters: todo!(),
    //         source: todo!(),
    //         symbol_rate: todo!(),
    //         video_pid: todo!(),
    //         audio_pid: todo!(),
    //         teletext_pid: todo!(),
    //         conditional_access: todo!(),
    //         service_id: todo!(),
    //         network_id: todo!(),
    //         transport_stream_id: todo!(),
    //         radio_id: todo!(),
    //     };

    //     let text = channel.format();

    //     assert_eq!(text, "rdvb| Some channel !,rdvb;Rust")
    // }
}

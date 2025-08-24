//! Frequency bands
//!
//! Parameters that define a band for broadcast of for TV services.
//! This allows scanning the most likely frequencies where a transponder could be found at.
//!
//! A user would probably want to choose their current country's parameters to get correct results while scanning.

use crate::frontend::properties::set::BandwidthHz;

// TODO: A way to chain multiple systems, like VHF then UHF, not just `.chain()` when scanning, to have a const here

// https://en.wikipedia.org/wiki/Band_IV
// https://en.wikipedia.org/wiki/Band_V
// https://www.tvnt.net/forum/tableau-de-conversion-des-canaux-uhf-en-frequences-t23059.html
// https://fr.wikipedia.org/wiki/Bandes_de_fr%C3%A9quences_de_la_t%C3%A9l%C3%A9vision_terrestre

//
// -----

/// Parameters for a single frequency slot
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ChannelParameters {
    /// Corresponding frequency
    pub frequency: u32,
    /// Bandwidth of this channel
    pub bandwidth: BandwidthHz,
    /// "Traditional" numbering for this channel
    pub number: Option<u32>,
    /// Optional prefix to show before "traditional" channel number
    pub display_prefix: &'static str,
}

/// Contiguous region of frequency "slots"
#[derive(Clone, Debug)]
pub struct BroadcastBand {
    pub first_frequency: u32,
    pub first_channel: u32,
    pub last_channel: u32,
    pub bandwidth: BandwidthHz,
    pub display_prefix: &'static str,
}

impl BroadcastBand {
    /// Return the amount of channels in this band
    pub fn channel_count(&self) -> u32 {
        self.last_channel - self.first_channel + 1
    }

    /// Iterate over all frequencies
    pub fn iter(&self) -> FrequencyIter<'_> {
        FrequencyIter {
            band: self,
            current_channel: self.first_channel,
        }
    }
}

/// Iterator for frequencies. This is used by [BroadcastBand::iter].
pub struct FrequencyIter<'a> {
    band: &'a BroadcastBand,
    current_channel: u32,
}

impl<'a> Iterator for FrequencyIter<'a> {
    type Item = ChannelParameters;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_channel > self.band.last_channel {
            return None;
        }

        let frequency = self.band.first_frequency
            + (self.current_channel - self.band.first_channel) * self.band.bandwidth.value();
        let number = Some(self.current_channel);

        self.current_channel += 1;

        Some(ChannelParameters {
            frequency,
            bandwidth: self.band.bandwidth,
            number,
            display_prefix: self.band.display_prefix,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.band.channel_count() as usize,
            Some(self.band.channel_count() as usize),
        )
    }
}

//
// -----

// --- Europe

pub const EUROPE_VHF_BAND_III: BroadcastBand = BroadcastBand {
    first_frequency: 177_500_000,
    first_channel: 5,
    last_channel: 12,
    bandwidth: BandwidthHz::_7MHz,
    display_prefix: "E",
};

pub const EUROPE_UHF_BAND_IV_V: BroadcastBand = BroadcastBand {
    first_frequency: 474_000_000,
    first_channel: 21,
    last_channel: 68,
    bandwidth: BandwidthHz::_8MHz,
    display_prefix: "",
};

// --- France

pub const FRANCE_CORRECTION: u32 = 166_000;

pub const FRANCE_UHF: BroadcastBand = BroadcastBand {
    first_frequency: EUROPE_UHF_BAND_IV_V.first_frequency + FRANCE_CORRECTION,
    last_channel: 49,
    ..EUROPE_UHF_BAND_IV_V
};

//
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let frequencies: Vec<ChannelParameters> = FRANCE_UHF.iter().collect();

        let first = ChannelParameters {
            frequency: 474_166_000,
            bandwidth: BandwidthHz::_8MHz,
            number: Some(21),
            display_prefix: "",
        };

        let expected = vec![
            first,
            ChannelParameters {
                frequency: 482_166_000,
                number: Some(22),
                ..first
            },
            ChannelParameters {
                frequency: 490_166_000,
                number: Some(23),
                ..first
            },
            ChannelParameters {
                frequency: 498_166_000,
                number: Some(24),
                ..first
            },
            ChannelParameters {
                frequency: 506_166_000,
                number: Some(25),
                ..first
            },
            ChannelParameters {
                frequency: 514_166_000,
                number: Some(26),
                ..first
            },
            ChannelParameters {
                frequency: 522_166_000,
                number: Some(27),
                ..first
            },
            ChannelParameters {
                frequency: 530_166_000,
                number: Some(28),
                ..first
            },
            ChannelParameters {
                frequency: 538_166_000,
                number: Some(29),
                ..first
            },
            ChannelParameters {
                frequency: 546_166_000,
                number: Some(30),
                ..first
            },
            ChannelParameters {
                frequency: 554_166_000,
                number: Some(31),
                ..first
            },
            ChannelParameters {
                frequency: 562_166_000,
                number: Some(32),
                ..first
            },
            ChannelParameters {
                frequency: 570_166_000,
                number: Some(33),
                ..first
            },
            ChannelParameters {
                frequency: 578_166_000,
                number: Some(34),
                ..first
            },
            ChannelParameters {
                frequency: 586_166_000,
                number: Some(35),
                ..first
            },
            ChannelParameters {
                frequency: 594_166_000,
                number: Some(36),
                ..first
            },
            ChannelParameters {
                frequency: 602_166_000,
                number: Some(37),
                ..first
            },
            ChannelParameters {
                frequency: 610_166_000,
                number: Some(38),
                ..first
            },
            ChannelParameters {
                frequency: 618_166_000,
                number: Some(39),
                ..first
            },
            ChannelParameters {
                frequency: 626_166_000,
                number: Some(40),
                ..first
            },
            ChannelParameters {
                frequency: 634_166_000,
                number: Some(41),
                ..first
            },
            ChannelParameters {
                frequency: 642_166_000,
                number: Some(42),
                ..first
            },
            ChannelParameters {
                frequency: 650_166_000,
                number: Some(43),
                ..first
            },
            ChannelParameters {
                frequency: 658_166_000,
                number: Some(44),
                ..first
            },
            ChannelParameters {
                frequency: 666_166_000,
                number: Some(45),
                ..first
            },
            ChannelParameters {
                frequency: 674_166_000,
                number: Some(46),
                ..first
            },
            ChannelParameters {
                frequency: 682_166_000,
                number: Some(47),
                ..first
            },
            ChannelParameters {
                frequency: 690_166_000,
                number: Some(48),
                ..first
            },
            ChannelParameters {
                frequency: 698_166_000,
                number: Some(49),
                ..first
            },
        ];

        assert_eq!(frequencies, expected)
    }
}

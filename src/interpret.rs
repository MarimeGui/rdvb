//! Interpret data received from SI into more useable things, like a channel config file.

use crate::{
    conf::vdr::{
        audio_pid::{AudioPID, AudioPIDList},
        video_pid::VideoPID,
    },
    frontend::{properties::set::BandwidthHz, sys::FeDeliverySystem},
    mpeg::{decode_stupid_string, descriptors::Descriptor},
    scan::Transponder,
    si::{
        nit::{NetworkInformationTable, NitElement},
        pmt::{ProgramMapTable, StreamType},
    },
};

/// A single logical channel, as in an actual TV channel.
///
/// This is available after analysis of the transponder data received from the air.
#[derive(Clone, Debug)]
pub struct ChannelInformation {
    pub frequency: u32,
    pub bandwidth: BandwidthHz,
    pub delivery_system: FeDeliverySystem,
    pub symbol_rate: Option<u32>,
    pub name: String,
    pub logical_channel_number: Option<u16>,
    pub service_id: u16,
    pub original_network_id: u16,
    pub transport_stream_id: u16,
    pub video_pid: VideoPID, // TODO: Should have own generic types instead of using VDR ones
    pub audio_pid_list: AudioPIDList,
}

impl ChannelInformation {
    /// Get all channels from a single transponder
    pub fn from_transponder(transponder: &Transponder) -> Vec<ChannelInformation> {
        let mut channels = Vec::new();

        for service in &transponder.service_description_table.services {
            // Find the service descriptor
            // TODO: Being able to store that specific descriptor would be easier
            let mut service_descriptor = None;
            for descriptor in &service.descriptors {
                if let Descriptor::Service(service) = descriptor {
                    service_descriptor = Some(service);
                    break;
                }
            }

            let service_data = if let Some(d) = service_descriptor {
                d
            } else {
                // No service descriptor, no idea what this service is about
                continue;
            };
            let name = service_data.service.clone();

            // Match corresponding NITElement
            let nit_element = if let Some(e) = find_nit_element_by_service_id(
                &transponder.network_information_table,
                service.service_id,
            ) {
                e
            } else {
                // Weird that this service isn't in the NIT, skip it
                continue;
            };

            let pmt_element = if let Some(e) =
                find_pmt_by_service_id(&transponder.program_map, service.service_id)
            {
                e
            } else {
                continue;
            };

            let logical_channel_number =
                find_lcn_from_nit_element_by_service_id(nit_element, service.service_id);

            channels.push(ChannelInformation {
                frequency: transponder.frequency,
                bandwidth: transponder.bandwidth,
                delivery_system: transponder.system,
                symbol_rate: None, // TODO: Symbol rate properly
                name,
                logical_channel_number,
                service_id: service.service_id,
                original_network_id: transponder.service_description_table.original_network_id,
                transport_stream_id: nit_element.transport_stream_id,
                video_pid: pmt_to_video_pid(pmt_element).unwrap(),
                audio_pid_list: pmt_to_audio_pids(pmt_element),
            })
        }

        channels
    }
}

/// Takes all found transponders during scan and returns a nice list of channels
pub fn to_channels(all_transponders: &[Transponder]) -> Vec<ChannelInformation> {
    let mut channels = Vec::new();

    for transponder in all_transponders {
        channels.append(&mut ChannelInformation::from_transponder(transponder));
    }

    channels
}

/// Sort a list of channels by their logical channel
pub fn sort_by_lcn(channels: &mut [ChannelInformation]) {
    channels.sort_by(
        |a, b| match (a.logical_channel_number, b.logical_channel_number) {
            (Some(a), Some(b)) => a.cmp(&b),
            (Some(_), _) => std::cmp::Ordering::Greater,
            (_, Some(_)) => std::cmp::Ordering::Less,
            (_, _) => std::cmp::Ordering::Equal,
        },
    );
}

fn find_nit_element_by_service_id(
    nit: &NetworkInformationTable,
    service_id: u16,
) -> Option<&NitElement> {
    for element in &nit.elements {
        for descriptor in &element.transport_descriptors {
            if let Descriptor::ServiceList(service_list) = descriptor {
                for e in &service_list.services {
                    if e.service_id == service_id {
                        return Some(element);
                    }
                }
            }
        }
    }

    None
}

fn find_lcn_from_nit_element_by_service_id(
    nit_elements: &NitElement,
    service_id: u16,
) -> Option<u16> {
    for descriptor in &nit_elements.transport_descriptors {
        let logical_channel = if let Descriptor::LogicalChannel(l) = descriptor {
            l
        } else {
            continue;
        };

        for lc_element in &logical_channel.elements {
            if lc_element.service_id == service_id {
                return Some(lc_element.logical_channel_number);
            }
        }
    }

    None
}

fn find_pmt_by_service_id(
    program_map: &[ProgramMapTable],
    service_id: u16,
) -> Option<&ProgramMapTable> {
    program_map.iter().find(|&e| e.program_number == service_id)
}

// TODO: Could merge all PID searches into a single fn

fn pmt_to_video_pid(pmt_element: &ProgramMapTable) -> Option<VideoPID> {
    // Search through all Elementary Streams and look for Video streams
    for elementary_stream in &pmt_element.elementary_streams {
        // Skip non-video streams
        if !elementary_stream.stream_type.is_video() {
            continue;
        }

        // Check if video PID is different from PCR
        let video_pid = if pmt_element.pcr_pid != elementary_stream.elementary_pid {
            Some(elementary_stream.elementary_pid)
        } else {
            None
        };

        return Some(VideoPID {
            pcr_pid: pmt_element.pcr_pid,
            video_pid,
            video_mode: elementary_stream.stream_type.to_u8() as u16,
        });
    }

    None
}

fn pmt_to_audio_pids(pmt_element: &ProgramMapTable) -> AudioPIDList {
    let mut regular_pids = Vec::new();
    let mut dolby_pids = Vec::new();

    // Same strategy as w_scan2 scan.c parse_pmt
    for elementary_stream in &pmt_element.elementary_streams {
        // Find language code for audio if any
        let mut language_code = String::new();
        for descriptor in &elementary_stream.descriptors {
            if let Descriptor::Iso639Language(lang) = descriptor {
                // TODO: This may not be in the same encoding, idk
                language_code = decode_stupid_string(&lang.language).unwrap()
            }
        }

        match &elementary_stream.stream_type {
            // Regular Audio
            StreamType::IsoIec11172Audio | StreamType::IsoIec13818_3Audio => {
                regular_pids.push(AudioPID {
                    pid: elementary_stream.elementary_pid,
                    language_code,
                    second_language_code: String::new(), // TODO: Not sure where the secondary language codes come from
                    audio_type: Some(elementary_stream.stream_type.to_u8() as u16),
                });
            }

            // Enhanced (Dolby) Audio
            StreamType::ItuTRecH2220IsoIec13818_1PrivateSections
            | StreamType::ItuTRecH2220IsoIec13818_1PESPacketsContainingPrivateData => {
                // Further check if this stream actually contains audio by checking descriptors
                // TODO: This does not work for AC4, as there is no new descriptor. Seems like Extension is used instead.
                let mut audio_type = None;
                for descriptor in &elementary_stream.descriptors {
                    match descriptor {
                        Descriptor::Ac3(_) => {
                            audio_type = Some(descriptor.descriptor_id());
                            break;
                        }
                        Descriptor::EnhancedAc3(_) => audio_type = Some(descriptor.descriptor_id()),
                        _ => {}
                    }
                }
                let audio_type = if let Some(a) = audio_type {
                    a
                } else {
                    continue;
                };

                dolby_pids.push(AudioPID {
                    pid: elementary_stream.elementary_pid,
                    language_code,
                    second_language_code: String::new(),
                    // TODO: audio_type is weird, w_scan2 and other data I found isn't coherent
                    //audio_type: Some(elementary_stream.stream_type.to_u8() as u16),
                    audio_type: Some(audio_type as u16),
                });
            }

            // TODO: Remaining audio types
            // StreamType::IsoIec13818_7AudioWithAdtsTransportSyntax => {}
            // StreamType::IsoIec14496_3AudioWithTheLatmTransportSyntaxAsDefinedInIsoIec14496_3Amd1 => {}
            _ => {}
        }
    }

    AudioPIDList {
        regular_pids,
        dolby_pids,
    }
}

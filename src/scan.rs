//! Helpers for scanning a DVB system for channels or other information.

use std::{collections::HashMap, path::Path, time::Duration};

use crate::{
    bands::ChannelParameters,
    demux::{PidTableIdPair, receive_multiple_single_packets, receive_single_packet},
    frontend::{
        DeliverySystem, Frontend,
        properties::{get::SignalStrength, set::BandwidthHz},
    },
    si::{
        nit::{ACTUAL_NETWORK_TABLE_ID as NIT_ACTUAL_NETWORK_TABLE_ID, NetworkInformation},
        pat::{PID as PAT_PID, PatValue, TABLE_ID as PAT_TABLE_ID, parse_pat},
        pmt::{ProgramMap, TABLE_ID as PMT_TABLE_ID},
        sdt::{
            ACTUAL_TRANSPORT_TABLE_ID as SDT_ACTUAL_TRANSPORT_TABLE_ID, PID as SDT_PID,
            ServiceDescription,
        },
    },
};

const LOCK_TIMEOUT: Duration = Duration::from_secs(1);
const PAT_TIMEOUT: Duration = Duration::from_secs(3); // A bit longer as DVB-T2 seems to send these less often

/// A single physical transponder emitting DVB data out over a frequency for a system.
#[derive(Debug)]
pub struct Transponder {
    pub frequency: u32,
    pub system: DeliverySystem,
    pub bandwidth: BandwidthHz,
    pub strength: SignalStrength,
    pub program_map: Vec<ProgramMap>,
    pub service_description: ServiceDescription,
    pub network_information: NetworkInformation,
}

/// Scans a whole system, like DVB-T or DVB-S. This returns a list of valid transponders.
pub fn scan_system<F, T>(
    frontend: &mut Frontend,
    frequencies: T,
    system: DeliverySystem,
    demux_path: &Path,
    cb: F,
) -> Vec<Transponder>
where
    F: Fn(usize),
    T: Iterator<Item = ChannelParameters>,
{
    // Indexed by transport stream ID (unique per transponder)
    let mut found_transponders: HashMap<u16, Transponder> = HashMap::new();

    for channel in frequencies {
        scan_channel(
            frontend,
            demux_path,
            system,
            channel.frequency,
            channel.bandwidth,
            &mut found_transponders,
        );
        cb(found_transponders.len())
    }

    found_transponders.into_values().collect()
}

/// Scan a single channel (as in frequency, not TV channel) for a given system to look for a valid transponder.
///
/// This also checks for duplicate transponders.
pub fn scan_channel(
    frontend: &mut Frontend,
    demux_path: &Path,
    system: DeliverySystem,
    frequency: u32,
    bandwidth: BandwidthHz,
    found_transponders: &mut HashMap<u16, Transponder>,
) {
    // --- Tune to given frequency, bandwidth and system
    // TODO: No need to set bandwidth and system every time, right ?
    frontend.tune(frequency, system, bandwidth).unwrap();

    // --- Check every 100ms if the frontend got a lock on something
    if !frontend.wait_for_lock(Some(LOCK_TIMEOUT), None).unwrap() {
        return;
    }

    // --- Get the PAT (Program Association Table) on its own
    let packet =
        match receive_single_packet(demux_path, PAT_PID, Some(PAT_TABLE_ID), Some(PAT_TIMEOUT)) {
            Ok(v) => v,
            Err(e) => match e.kind() {
                // If receiving a valid packet times out, this probably means we're not receiving this transponder well enough, skip it
                std::io::ErrorKind::TimedOut => return,
                _ => panic!(),
            },
        };
    let pat_entries = parse_pat(&packet);
    let transport_stream_id = packet.header.identifier;

    // --- Query signal strength and compare with previously received transponder if some
    let strength = frontend.signal_strength().unwrap();
    if let Some(prev_transponder) = found_transponders.get(&transport_stream_id) {
        // We picked up the same transponder twice, choose the one with the strongest signal
        match strength.partial_cmp(&prev_transponder.strength) {
            Some(o) => match o {
                // This frequency has stronger reception, continue.
                std::cmp::Ordering::Greater => {}
                // The other was better or equal, don't continue with this one.
                _ => return,
            },
            // Trying to compare either incompatible units or an outright failure.
            // This should not happen unless I messed up or the adapter is hysteric
            None => panic!(),
        }
    }

    // --- Get modulation
    // TODO: Could merge with above, make a single fn for all relevant info
    // let mut modulation = Modulation::query();
    // frontend.properties(&mut [modulation.desc()]).unwrap();
    // let modulation = modulation.retrieve().unwrap().0;
    // println!("{:?}", modulation);

    // --- Get info about the transponder
    let mut all_pairs = Vec::new();

    // TODO: In theory, could use Table IDs to distinguish them instead
    // Add all PIDs from PAT
    let mut nit_indices = Vec::new();
    let mut pmt_indices = Vec::new();
    for entry in pat_entries {
        match entry.value {
            PatValue::Network(pid) => {
                nit_indices.push(all_pairs.len());
                all_pairs.push(PidTableIdPair {
                    pid,
                    table_id: Some(NIT_ACTUAL_NETWORK_TABLE_ID),
                });
            }
            PatValue::ProgramMap(pid) => {
                pmt_indices.push(all_pairs.len());
                all_pairs.push(PidTableIdPair {
                    pid,
                    table_id: Some(PMT_TABLE_ID),
                });
            }
        }
    }

    // Add SDT
    let sdt_index = all_pairs.len();
    all_pairs.push(PidTableIdPair {
        pid: SDT_PID,
        table_id: Some(SDT_ACTUAL_TRANSPORT_TABLE_ID),
    });

    // Receive all packets
    let packets = receive_multiple_single_packets(demux_path, all_pairs, None).unwrap();

    // Parse all NITs (there should only be one)
    // TODO: Could optimize this for a single packet...
    let mut nit = None;
    for index in nit_indices {
        nit = Some(NetworkInformation::from_packet(&packets[index]));
    }
    let nit = nit.unwrap();

    // Parse all PMTs
    let mut program_map = Vec::new();
    for index in pmt_indices {
        let pmt = ProgramMap::from_packet(&packets[index]);
        program_map.push(pmt);
    }

    // Parse SDT
    let sdt = ServiceDescription::from_packet(&packets[sdt_index]);

    found_transponders.insert(
        transport_stream_id,
        Transponder {
            frequency,
            system,
            bandwidth,
            strength,
            program_map,
            service_description: sdt,
            network_information: nit,
        },
    );
}

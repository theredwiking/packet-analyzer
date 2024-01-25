use pnet::packet::{ethernet::{EtherTypes, EthernetPacket}, ipv4::Ipv4Packet, ipv6::Ipv6Packet, Packet};
use crate::{influx::RawPacket, listener::PacketInfo};
use influxdb::{InfluxDbWriteable, WriteQuery};
use chrono::offset;

// TODO: Delete struct
#[allow(dead_code)]
#[derive(Debug)]
pub struct PacketConvert {
    ipv: Box<str>,
    source_ip: String,
    dest_ip: String,
    protocol: String,
    //payload: Box<[u8]>,
}

pub fn ip_version(packet: PacketInfo) -> Option<WriteQuery> {
    if let Some(ethernet_packer) = EthernetPacket::new(&packet.data) {
        let current_time = offset::Utc::now();
        match ethernet_packer.get_ethertype() {
            EtherTypes::Ipv4 => {
                let ipv4_packet = Ipv4Packet::new(ethernet_packer.payload()).unwrap();
                let converted = RawPacket {
                    time: current_time,
                    source: ipv4_packet.get_source().to_string(),
                    destination: ipv4_packet.get_destination().to_string(),
                    protocol: ipv4_packet.get_next_level_protocol().to_string(),
                    //payload: ipv4_packet.payload().into(),
                };
                Some(converted.into_query("ipv4"))
            },
            EtherTypes::Ipv6 => {
                let ipv6_packet = Ipv6Packet::new(ethernet_packer.payload()).unwrap();
                let converted = RawPacket {
                    time: current_time,
                    source: ipv6_packet.get_source().to_string(),
                    destination: ipv6_packet.get_destination().to_string(),
                    protocol: ipv6_packet.get_next_header().to_string(),
                    //payload: ipv6_packet.payload().into(),
                };
                Some(converted.into_query("ipv6"))
            },
            EtherTypes::Arp => {None},
            _ => {println!("Unsupported ip version: {}", ethernet_packer.get_ethertype()); None}
        }
    } else {
        None
    }
}

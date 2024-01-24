use std::u8;

use pnet::packet::{ethernet::{EtherTypes, EthernetPacket}, ipv4::Ipv4Packet, ipv6::Ipv6Packet, Packet};
use crate::listener::PacketInfo;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PacketConvert {
    ipv: Box<str>,
    source_ip: String,
    dest_ip: String,
    protocol: String,
    payload: Box<[u8]>,
}

pub fn ip_version(packet: PacketInfo) {
    if let Some(ethernet_packer) = EthernetPacket::new(&packet.data) {
        match ethernet_packer.get_ethertype() {
            EtherTypes::Ipv4 => {
                let ipv4_packet = Ipv4Packet::new(ethernet_packer.payload()).unwrap();
                let converted = PacketConvert {
                    ipv: "4".into(),
                    source_ip: ipv4_packet.get_source().to_string(),
                    dest_ip: ipv4_packet.get_destination().to_string(),
                    protocol: ipv4_packet.get_next_level_protocol().to_string(),
                    payload: ipv4_packet.payload().into(),
                };
                println!("{:?}", converted)
            },
            EtherTypes::Ipv6 => {
                let ipv6_packet = Ipv6Packet::new(ethernet_packer.payload()).unwrap();
                let converted = PacketConvert {
                    ipv: "6".into(),
                    source_ip: ipv6_packet.get_source().to_string(),
                    dest_ip: ipv6_packet.get_destination().to_string(),
                    protocol: ipv6_packet.get_next_header().to_string(),
                    payload: ipv6_packet.payload().into(),
                };
                println!("{:?}", converted)
            },
            _ => {}
        }
    }
}

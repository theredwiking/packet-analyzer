use pnet::packet::{ethernet::{EtherTypes, EthernetPacket}, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, Packet};
use crate::listener::PacketInfo;

#[derive(Debug)]
pub struct PacketConvert {
    source_ip: String,
    dest_ip: String,
    protocol: String,
}

pub fn ip_version(packet: PacketInfo) {
    if let Some(ethernet_packer) = EthernetPacket::new(&packet.data) {
        match ethernet_packer.get_ethertype() {
            EtherTypes::Ipv4 => {
                let ipv4_packet = Ipv4Packet::new(ethernet_packer.payload()).unwrap();
                let converted = PacketConvert {
                    source_ip: ipv4_packet.get_source().to_string(),
                    dest_ip: ipv4_packet.get_destination().to_string(),
                    protocol: ipv4_packet.get_next_level_protocol().to_string(),
                };
                println!("{:?}", converted)
            },
            _ => {}
        }
    }
}

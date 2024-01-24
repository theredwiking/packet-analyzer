use pnet::packet::{ethernet::{EtherTypes, EthernetPacket}, ipv4::Ipv4Packet, Packet};
use crate::listener::PacketInfo;

pub fn ip_version(packet: PacketInfo) {
    if let Some(ethernet_packer) = EthernetPacket::new(&packet.data) {
        match ethernet_packer.get_ethertype() {
            EtherTypes::Ipv4 => {
                let ipv4_packet = Ipv4Packet::new(ethernet_packer.payload()).unwrap();
                let src_ip = ipv4_packet.get_source().to_string();
                let dst_ip = ipv4_packet.get_destination().to_string();
                let packet_protocol = ipv4_packet.get_next_level_protocol().to_string();
                println!("From: {0}, To: {1}, Protocol: {2}", src_ip, dst_ip, packet_protocol);
            },
            _ => {}
        }
    }
}

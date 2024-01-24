use pcap::{Capture, PacketHeader};
use tokio::sync::mpsc::Sender;
use std::u8;

#[derive(Debug, Clone)]
pub struct PacketInfo {
    pub header: PacketHeader,
    pub data: Box<[u8]>,
}

pub async fn listen(interace: &str, sender: Sender<PacketInfo>) {
    let mut cap = Capture::from_device(interace).unwrap()
        .promisc(true)
        .snaplen(262144)
        .open().unwrap();
    while let Ok(packet) = cap.next_packet()  {
        let _ = sender.send(
            PacketInfo {
                header: *packet.header,
                data: packet.data.into(),
            }
        ).await;
    }
}

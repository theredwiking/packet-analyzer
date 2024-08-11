use pcap::Device;

pub fn list() {
    for device in Device::list().expect("Failed to lookup devices") {
        if !device.addresses.is_empty() {
            println!("Found new device name: {0}", device.name);
            println!("Addresses:");
            for address in device.addresses {
                println!("{:?}", address.addr);
            }
            println!("<--------------------------------------->");
        }
    }
}

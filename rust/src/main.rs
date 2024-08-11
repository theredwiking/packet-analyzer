mod classifier;
mod listener;
mod devices;
mod config;
mod influx;

use config::{load_config, Config};
use clap::{Parser, ArgAction};
use classifier::ip_version;
use influxdb::WriteQuery;
use tokio::sync::mpsc;
use listener::listen;
use influx::database;
use devices::list;

#[derive(Parser)]
struct Args {
    /// Interface to listen on
    #[arg(short, long, default_value = "")]
    interface: String,
    /// Get list of interfaces
    #[arg(short, long, action=ArgAction::SetTrue)]
    devices: bool,
}

#[tokio::main]
async fn main() {
    let config: Config = load_config().expect("Failed to load");

    let args = Args::parse();

    let (tx, mut rx) = mpsc::channel(255);

    if args.devices {
        list();
    } else {
        tokio::spawn(async move {
            let mut packets: Vec<WriteQuery> = vec![];
            while let Some(packet) = rx.recv().await {
                if packets.len() == 500 {
                    database(config.database.clone(), &packets).await;
                    packets.clear();
                }
                if let Some(converted) = ip_version(packet) {
                    packets.push(converted);
                }
            }
        });

        let interface = if !args.interface.is_empty() { args.interface.as_str() } else { config.network.interface.as_str() };
        listen(interface, tx).await;
    }
}

mod classifier;
mod listener;
mod devices;
mod config;

use clap::{Parser, ArgAction};
use classifier::ip_version;
use config::{load_config, Config};
use tokio::sync::mpsc;
use listener::listen;
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
            while let Some(packet) = rx.recv().await {
                ip_version(packet);
            }
        });

        let interface = if !args.interface.is_empty() { args.interface.as_str() } else { config.network.interface.as_str() };
        listen(interface, tx).await;
    }
}

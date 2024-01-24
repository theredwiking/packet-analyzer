mod classifier;
mod listener;
mod devices;

use clap::{Parser, ArgAction};
use classifier::ip_version;
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
    let args = Args::parse();

    let (tx, mut rx) = mpsc::channel(255);

    if args.devices {
        list();
    } else {
        tokio::spawn(async move {
            listen(args.interface.as_str(), tx).await;
        });

        while let Some(packet) = rx.recv().await {
            ip_version(packet);
        }
    }
}

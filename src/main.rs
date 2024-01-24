mod devices;
mod listener;
mod classifier;

use clap::{Parser, ArgAction};
use tokio::sync::mpsc;
use listener::listen;
use devices::list;
use classifier::ip_version;

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

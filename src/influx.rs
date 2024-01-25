use crate::config::DBConfig;
use chrono::{offset, DateTime, Utc};
use influxdb::{Client, InfluxDbWriteable};

#[derive(InfluxDbWriteable)]
struct RawPacket {
    time: DateTime<Utc>,
    #[influxdb(tag)]protocol: String,
    source: String,
    destination: String,
}

pub async fn database(config: DBConfig) {
    let client = Client::new(config.url, config.bucket).with_token(config.token);
    let current_time = offset::Utc::now();

    let packet = RawPacket {
        time: current_time,
        source: String::from("1.1.1.1"),
        destination: String::from("192.168.0.1"),
        protocol: String::from("Tcp"),
    }.into_query("packer");

    let write_result = client
        .query(packet)
        .await;
    assert!(write_result.is_ok(), "Writing failed");
}

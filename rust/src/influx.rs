use influxdb::{Client, InfluxDbWriteable, WriteQuery};
use crate::config::DBConfig;
use chrono::{DateTime, Utc};

// TODO: Find way to store payload u8
#[derive(InfluxDbWriteable, Clone)]
pub struct RawPacket {
    pub time: DateTime<Utc>,
    #[influxdb(tag)] pub protocol: String,
    pub source: String,
    pub destination: String,
}

pub async fn database(config: DBConfig, packets: &Vec<WriteQuery>) {
    let client = Client::new(config.url, config.bucket).with_token(config.token);
    
    let write_result = client
        .query(packets)
        .await;
    assert!(write_result.is_ok(), "Writing failed");
}

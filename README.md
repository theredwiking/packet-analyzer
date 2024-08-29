# Packet-Analyzer

This is an project for learning about packet sniffing using go.

It is also used to test and learn different techologies, some of the them will probably be:
- [Prometheus](https://prometheus.io/)
- [Influxdb](https://www.influxdata.com/)
- [OpenTelemetry](https://opentelemetry.io/)
- [Grafana](https://grafana.com/)

## Needs to be tested
Must be tested in golang project\
Prometheus vs OpenTelemetry (for metrics)\
Logrus vs Zap (save log files to influxdb)\
Log library vs OpenTelemetry (for logs)

## Missing information
Where to save OpenTelemetry data, both logs and metric.\
Display logs and metrics in Grafana.

# Tested enviroments
This project is fully developed and tested on Debian.\
Help need for Windows and MacOS development part and test

# Setup

## Requiments
- go version 1.22 or newer
- libpcap-dev
- setcap

## Compile
```bash
go build -v -o dist/sniffer ./cmd/sniffer
```

## To run
```bash
sudo setcap cap_net_raw,cap_net_admin=eip ./dist/sniffer
```

# Config
```toml
[network]
interface = "<eht0>"
snaplen = <262144>
promiscuous = <false>

[database]
url = "http://<ip>:<port>"
bucket = "<bucket>"
token = "<apikey>"
```

# Todo:
- [ ] Add struct for easy handle of packet data
- [ ] Decoded packet data into custom struct
- [ ] Save data to influxdb
- [ ] Research sending data to kafka for investigation
- [ ] Add better test
- [ ] Add performance monitoring
- [ ] Figure out how to package program for different distros
- [ ] Add more docs

# Packet-Analyzer

This is an project for learning about packet sniffing using go.\
This project can also be used by other go projects as an package, for how ton use look at cmd/sniffer/sniffer.go for implementation of functions

It is also used to test and learn different techologies like influxdb and perhaps kafka

## Info
This project is fully developed and tested on Debian.

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
This project is fully developed and tested on Debian.

# Setup

## Requiments
- go version 1.22 or newer
- libpcap-dev
- setcap

## Compile
If make is installed make use of makefile only for linux currently
```bash
make
```

Else use this command to compile for current platform
```bash
go build -v -o dist/sniffer ./cmd/sniffer
```

## To run
Makefile also containts option for running development 
```bash
make dev
```

If above command was not used this is necesarry
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
- [ ] Add function and channel to handle packet data
- [ ] Add better test
- [ ] Add performance monitoring
- [ ] Figure out how to package program for different distros
- [ ] Add more docs

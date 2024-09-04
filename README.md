# Packet-Analyzer

This is an project for learning about packet sniffing using go.
It is also used to test and learn different techologies like influxdb and perhaps kafka

## Info
This project is fully developed and tested on Debian.
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
- [ ] Add function and channel to handle packet data
- [ ] Add better test
- [ ] Add performance monitoring
- [ ] Figure out how to package program for different distros
- [ ] Add more docs

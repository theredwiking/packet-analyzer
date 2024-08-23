# packet-analyzer go

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
1. Add model to handle packet data
2. Add influxdb connection
3. Handle incoming packets
4. Look into testing(Probably needs to rewrite some functions)

# packet-analyzer go

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

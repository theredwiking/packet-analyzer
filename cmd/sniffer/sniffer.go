package main

import (
	"flag"
	"fmt"

	"github.com/theredwiking/packet-analyzer/go/pkg/config"
	"github.com/theredwiking/packet-analyzer/go/pkg/interfaces"
	"github.com/theredwiking/packet-analyzer/go/pkg/listener"
)

func main() {
	ifaces := flag.Bool("i", false, "List available interfaces")
	configFile := flag.Bool("c", false, "List out config content")

	flag.Parse()

	conf := config.LoadConfig()

	if *ifaces {
		interfaces.InterfaceList()
	}

	if *configFile {
		conf.Contains()
	}

	if !*ifaces && !*configFile {
		fmt.Printf("Starting to listing on interface: %s", conf.Iface)
		listener.StartListener(*conf)
	}
}

package main

import (
	"flag"
	"fmt"

	"github.com/theredwiking/packet-analyzer/go/core"
	"github.com/theredwiking/packet-analyzer/go/lib"
)

func main() {
	interfaces := flag.Bool("i", false, "List available interfaces")
	config := flag.Bool("c", false, "List out config content")

	flag.Parse()

	conf := lib.LoadConfig()

	if *interfaces {
		core.InterfaceList()
	}

	if *config {
		conf.Contains()
	}

	if !*interfaces && !*config {
		fmt.Printf("Starting to listing on interface: %s", conf.Iface)
		core.StartListiner(*conf)
	}
}

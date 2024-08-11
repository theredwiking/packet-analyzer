package main

import (
	"flag"

	"github.com/theredwiking/packet-analyzer/go/core"
)

func main() {
	interfaces := flag.Bool("i", false, "List avaible interfaces")
	
	flag.Parse()

	if *interfaces {
		core.InterfaceList()
	}
}

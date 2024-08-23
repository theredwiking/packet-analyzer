package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/theredwiking/packet-analyzer/go/pkg/config"
	"github.com/theredwiking/packet-analyzer/go/pkg/interfaces"
	"github.com/theredwiking/packet-analyzer/go/pkg/listener"
)

func main() {
	ifaces := flag.Bool("i", false, "List available interfaces")
	configFile := flag.Bool("c", false, "List out config content")
	createConfig := flag.Bool("n", false, "Creates new config file")

	flag.Parse()

	conf, err := config.LoadConfig()
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	if *ifaces {
		interfaces.InterfaceList()
	}

	if *createConfig {
		err = config.CreateConfig("./config")
		if err != nil {
			fmt.Println(err)
			return
		}
		fmt.Println("New config file created")
	}

	if *configFile {
		conf.Contains()
	}

	if !*ifaces && !*configFile && !*createConfig {
		fmt.Printf("Starting to listing on interface: %s", conf.Iface)
		listener.StartListener(*conf)
	}
}

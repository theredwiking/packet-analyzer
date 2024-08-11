package core

import (
	"fmt"
	"log"
	"net"
	"os"
	"text/tabwriter"
)

func InterfaceList() {
	ifaces, err := net.Interfaces()
	if err != nil {
		log.Println("Failed to interfaces")
		os.Exit(1)
	}

	w := tabwriter.NewWriter(os.Stdout,	0, 1, 1, ' ', 0)
	fmt.Fprintln(w, "Name\tAddress")

	for _, iface := range ifaces {
		if iface.Flags&net.FlagUp == 0 {
			continue
		}
		
		addrs, err := iface.Addrs()
		if err != nil {
			continue
		}

		fmt.Fprintf(w, "%s\t%s\n", iface.Name, getIP(addrs))
	}
	w.Flush()
}

func getIP(addrs []net.Addr) string {
	var ip string
	for _, addr := range addrs {
		var temp net.IP
		switch v := addr.(type) {
		case *net.IPNet:
			temp = v.IP
		case *net.IPAddr:
			temp = v.IP
		}
		if temp == nil {
			continue
		}
		temp = temp.To4()
		if temp == nil {
			continue
		}
		ip = temp.String()
	}

	return ip
}

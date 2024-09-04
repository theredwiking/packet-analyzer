package handlers

import (
	"fmt"

	"github.com/google/gopacket"
)

func terminal(packets chan gopacket.Packet) {
	for {
		select {
		case packet := <-packets:
			fmt.Println(packet.Dump())
		}
	}
}

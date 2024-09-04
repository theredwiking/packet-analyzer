package handlers

import (
	"fmt"

	"github.com/google/gopacket"
)

// Prints packet data directly in terminal
func terminal(packets chan gopacket.Packet) {
	for {
		select {
		case packet := <-packets:
			fmt.Println(packet.Dump())
		}
	}
}

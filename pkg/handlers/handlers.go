package handlers

import (
	"log"

	"github.com/google/gopacket"
)

// Select what handler to use and returns channel for sending packets
func Handler(handler int) chan gopacket.Packet {
	packets := make(chan gopacket.Packet, 64)

	switch handler {
	case 1:
		go terminal(packets)
	default:
		log.Println("No option provided")
	}
	return packets
}

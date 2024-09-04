package handlers

import (
	"log"

	"github.com/google/gopacket"
)

// Function to select how to handle incoming packets
func Handler(handler int, packets chan gopacket.Packet) {

	switch handler {
	case 1:
		go terminal(packets)
	default:
		log.Println("No option provided")
	}
}

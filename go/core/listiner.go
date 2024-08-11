package core

import (
	"log"
	"time"

	"github.com/google/gopacket"
	"github.com/google/gopacket/pcap"
	"github.com/theredwiking/packet-analyzer/go/models"
)

// Start listiner on the given interface
func StartListiner(config models.Config) {
	handle, err := pcap.OpenLive(config.Iface, config.Snaplen, config.Promiscuous, 30*time.Second)
	if err != nil {
		log.Fatalln(err)
	}

	defer handle.Close()

	packetSource := gopacket.NewPacketSource(handle, handle.LinkType())
	for packet := range packetSource.Packets() {
		log.Println(packet)
	}
}

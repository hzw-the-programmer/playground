package main

import (
	"log"
	"net"
)

func main() {
	wc, rc := StartUDPServer(":1123")
	for pkt := range rc {
		log.Printf("%s: %s", pkt.Addr, string(pkt.Data))
		wc <- pkt
	}
}

type UDPPkt struct {
	Addr net.Addr
	Data []byte
}

func StartUDPServer(addr string) (chan<- UDPPkt, <-chan UDPPkt) {
	conn, err := net.ListenPacket("udp", addr)
	if err != nil {
		log.Fatal(err)
	}

	wc := make(chan UDPPkt)
	go write(conn, wc)

	rc := make(chan UDPPkt)
	go read(conn, rc)

	return wc, rc
}

func write(conn net.PacketConn, wc <-chan UDPPkt) {
	for pkt := range wc {
		_, err := conn.WriteTo(pkt.Data, pkt.Addr)
		if err != nil {
			log.Print(err)
			break
		}
	}
}

func read(conn net.PacketConn, rc chan<- UDPPkt) {
	data := make([]byte, 1024)
	for {
		n, addr, err := conn.ReadFrom(data)
		if err != nil {
			log.Print(err)
			break
		}
		rc <- UDPPkt{addr, data[:n]}
	}
}

package main

import (
	"bufio"
	"flag"
	"log"
	"net"
	"os"
)

var port *int
var ip *string

func main() {
	isClient := flag.Bool("c", false, "is client or server")
	port = flag.Int("p", 3333, "port")
	ip = flag.String("i", "localhost", "remote ip")
	flag.Parse()

	if *isClient {
		client()
	} else {
		server()
	}
}

func client() {
	raddr := net.UDPAddr{IP: net.ParseIP(*ip), Port: *port}
	conn, err := net.DialUDP("udp", nil, &raddr)
	//conn, err := net.ListenUDP("udp", nil)
	if err != nil {
		log.Fatal(err)
	}
	go func(conn *net.UDPConn) {
		for {
			bs := make([]byte, 1024)
			rn, err := conn.Read(bs)
			//rn, raddr, err := conn.ReadFrom(bs)
			if err != nil {
				log.Fatal(err)
			}
			log.Printf("Read: %s", string(bs[:rn]))
			//log.Printf("ReadFrom: %s: %s", raddr, string(bs[:rn]))
		}
	}(conn)
	s := bufio.NewScanner(os.Stdin)
	for s.Scan() {
		wn, err := conn.Write(s.Bytes())
		//wn, err := conn.WriteTo(s.Bytes(), &raddr)
		if err != nil {
			log.Fatal(err)
		}
		if wn != len(s.Bytes()) {
			log.Fatal("short write")
		}
	}
}

func server() {
	laddr := net.UDPAddr{Port: *port}
	conn, err := net.ListenUDP("udp", &laddr)
	if err != nil {
		log.Fatal(err)
	}
	for {
		bs := make([]byte, 1024)
		rn, raddr, err := conn.ReadFrom(bs)
		if err != nil {
			log.Fatal(err)
		}
		log.Printf("ReadFrom: %s: %s", raddr, string(bs[:rn]))

		wn, err := conn.WriteTo(bs[:rn], raddr)
		if err != nil {
			log.Fatal(err)
		}
		if wn != rn {
			log.Fatal("short write")
		}
	}
}

package main

import (
	"bufio"
	"flag"
	"log"
	"net"
	"os"
	"encoding/hex"
)

var port *int
var ip *string
var lport *int

func main() {
	isClient := flag.Bool("c", false, "is client or server")
	port = flag.Int("p", 3333, "port")
	ip = flag.String("i", "127.0.0.1", "remote ip")
	lport = flag.Int("lp", 0, "port")
	flag.Parse()

	if *isClient {
		client()
	} else {
		server()
	}
}

func client() {
	raddr := net.UDPAddr{IP: net.ParseIP(*ip), Port: *port}
	laddr := net.UDPAddr{Port: *lport}
	//conn, err := net.DialUDP("udp", nil, &raddr)
	conn, err := net.ListenUDP("udp", &laddr)
	if err != nil {
		log.Fatal(err)
	}
	log.Print(conn.LocalAddr())
	
	go func(conn *net.UDPConn) {
		for {
			bs := make([]byte, 1024)
			//rn, err := conn.Read(bs)
			rn, raddr, err := conn.ReadFrom(bs)
			if err != nil {
				log.Fatal(err)
			}
			//log.Printf("Read: %s", string(bs[:rn]))
			log.Printf("ReadFrom: %s: %s", raddr, string(bs[:rn]))
		}
	}(conn)
	
	s := bufio.NewScanner(os.Stdin)
	for s.Scan() {
		//wn, err := conn.Write(s.Bytes())
		if (s.Text() == "laddr") {
			log.Print(conn.LocalAddr())
			continue
		}
		wn, err := conn.WriteTo(s.Bytes(), &raddr)
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
	clients := make(map[string]net.Addr)
	bs := make([]byte, 1024 * 1024)
	hs := make([]byte, 1024 * 1024)

	for {
		rn, raddr, err := conn.ReadFrom(bs)
		if err != nil {
			log.Fatal(err)
		}
		hex.Encode(hs, bs[:rn])

		log.Print("")
		log.Printf("******ReadFrom: %s:\n", raddr)
		log.Printf("******size    : %d", rn)
		log.Printf("******content : %s", string(hs[:2*rn]))
		
		clients[raddr.String()] = raddr

		log.Println(clients)

		for rs, r := range clients {
			log.Printf("------send to : %s", rs)
			if rs == raddr.String() {
				continue
			}
			wn, err := conn.WriteTo(bs[:rn], r)
			if err != nil {
				log.Fatal(err)
			}
			if wn != rn {
				log.Fatal("short write")
			}
		}
	}
}

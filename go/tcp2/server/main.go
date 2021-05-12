package main

import (
	"flag"
	"log"
	"net"
)

func main() {
	addr := flag.String("addr", ":1123", "server host:port")
	flag.Parse()
	log.SetFlags(log.Lshortfile)
	lis, err := net.Listen("tcp", *addr)
	if err != nil {
		log.Fatal(err)
	}
	for {
		conn, err := lis.Accept()
		if err != nil {
			log.Fatal(err)
		}
		go func() {
			data := make([]byte, 1024)
			for {
				n, err := conn.Read(data)
				if err != nil {
					log.Print(err)
					break
				}
				log.Print(string(data[:n]))
				n, err = conn.Write(data[:n])
				if err != nil {
					log.Print(err)
					break
				}
			}
		}()
	}
}

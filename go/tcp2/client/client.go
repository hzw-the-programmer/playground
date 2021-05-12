package main

import (
	"bufio"
	"flag"
	"log"
	"net"
	"os"
)

func main() {
	addr := flag.String("addr", "localhost:1123", "server host:port")
	flag.Parse()
	log.SetFlags(log.Lshortfile)
	conn, err := net.Dial("tcp", *addr)
	if err != nil {
		log.Fatal(err)
	}
	go func() {
		data := make([]byte, 1024)
		for {
			n, err := conn.Read(data)
			if err != nil {
				log.Fatal(err)
			}
			log.Print(string(data[:n]))
		}
	}()
	s := bufio.NewScanner(os.Stdin)
	for s.Scan() {
		conn.Write(s.Bytes())
	}
}

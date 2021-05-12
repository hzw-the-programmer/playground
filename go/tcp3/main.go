package main

import (
	"log"
	"net"
	"os"
)

func main() {
	port := ""
	if len(os.Args) > 1 {
		port = os.Args[1]
	}

	if port == "" {
		server()
	} else {
		client(port)
	}
}

func server() {
	ln, err := net.ListenTCP("tcp", nil)
	if err != nil {
		log.Fatal(err)
	}
	defer ln.Close()

	log.Print(ln.Addr())

	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Fatal(err)
		}
		go func() {
			defer conn.Close()
			log.Print(conn.LocalAddr())
			log.Print(conn.RemoteAddr())
		}()
	}
}

func client(port string) {
	raddr, err := net.ResolveTCPAddr("tcp", ":"+port)
	if err != nil {
		log.Fatal(err)
	}

	conn, err := net.DialTCP("tcp", nil, raddr)
	if err != nil {
		log.Fatal(err)
	}
	defer conn.Close()

	log.Print(conn.LocalAddr())
	log.Print(conn.RemoteAddr())
}

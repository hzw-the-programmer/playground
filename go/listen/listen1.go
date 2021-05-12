package main

import (
	"log"
	"net"
	"time"
)

func main() {
	ln, err := net.Listen("tcp", ":1123")
	if err != nil {
		log.Fatal(err)
	}
	go func() {
		time.Sleep(1 * time.Second)
		ln.Close()
	}()
	for {
		_, err := ln.Accept()
		if err != nil {
			log.Fatal(err)
		}
	}
}

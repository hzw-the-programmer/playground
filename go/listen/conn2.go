package main

import (
	"log"
	"net"
	"sync"
	"time"
)

var wg sync.WaitGroup

func serve(conn net.Conn) {
	defer wg.Done()
	defer func() {
		log.Printf("conn.Close(): %s", conn.Close())
	}()
	data := make([]byte, 1024)
	for {
		n, err := conn.Read(data)
		if err != nil {
			log.Printf("conn.Read(): %s", err)
			break
		}
		n, err = conn.Write(data[:n])
		if err != nil {
			log.Printf("conn.Write(): %s", err)
			break
		}
	}
}

func main() {
	ln, err := net.Listen("tcp", ":1123")
	if err != nil {
		log.Fatal(err)
	}
	defer func() {
		log.Printf("ln.Close(): %s", ln.Close())
	}()
	go func() {
		time.Sleep(10 * time.Second)
		log.Printf("ln.Close(): %s", ln.Close())
	}()
	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Printf("ln.Accept(): %s", err)
			break
		}
		wg.Add(1)
		go serve(conn)
	}
	wg.Wait()
}

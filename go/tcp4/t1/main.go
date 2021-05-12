package main

import (
	"log"
	"net"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup

	ln, err := net.Listen("tcp", ":8080")
	if err != nil {
		log.Fatal(err)
	}

	// wg.Add(1)
	go func() {
		// defer wg.Done()
		for {
			conn, err := ln.Accept()
			if err != nil {
				log.Fatal(err)
			}
			go func(conn net.Conn) {
				log.Printf("server: %s connected", conn.RemoteAddr())
			}(conn)
		}
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		time.Sleep(1*time.Second)
		conn, err := net.Dial("tcp", ":8080")
		if err != nil {
			log.Print(err)
			return
		}
		conn.SetReadDeadline(time.Now().Add(50 * time.Millisecond))
		start := time.Now()
		var data [1024]byte
		_, err = conn.Read(data[:])
		if err != nil {
			_, ok := err.(net.Error)
			log.Printf("type: %T, is net.Error: %v, error: %s, cost: %s", err, ok, err, time.Now().Sub(start))
		}
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		start := time.Now()
		_, err := net.DialTimeout("tcp", ":8081", 50*time.Millisecond)
		if err != nil {
			_, ok := err.(net.Error)
			log.Printf("type: %T, is net.Error: %v, error: %s, cost: %s", err, ok, err, time.Now().Sub(start))
		}
	}()

	wg.Wait()
}

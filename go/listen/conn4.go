package main

import (
	"log"
	"net"
	"time"
)

func serve(conn net.Conn) {
	defer func() {
		log.Print("bc1")
		log.Printf("c1: %s", conn.Close())
		log.Print("ac1")
	}()
	go func() {
		time.Sleep(2 * time.Second)
		log.Print("SetReadDeadline")
		conn.SetReadDeadline(time.Now().Add(1 * time.Second))
	}()
	go func() {
		time.Sleep(10 * time.Second)
		log.Print("bc2")
		log.Printf("c2: %s", conn.Close())
		log.Print("ac2")
	}()
	data := make([]byte, 1024)
	for {
		n, err := conn.Read(data)
		if err != nil {
			log.Print(err)
			if nerr, ok := err.(net.Error); ok {
				if nerr.Timeout() {
					log.Print("Timeout")
					conn.SetReadDeadline(time.Time{})
					continue
				} else if nerr.Temporary() {
					log.Print("Temporay")
				}
			}
			break
		}
		log.Print(string(data[:n]))
	}
}

func main() {
	ln, err := net.Listen("tcp", ":1123")
	if err != nil {
		log.Fatal(err)
	}
	defer ln.Close()
	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Print(err)
			break
		}
		go serve(conn)
	}
}

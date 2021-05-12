package main

import (
	"log"
	"net"
	"time"
)

func serve(conn net.Conn) {
	defer func(){
		log.Printf("defer: %s", conn.Close())
	}()
	go func() {
		time.Sleep(1 * time.Second)
		log.Print("before conn.Close() 1")
		log.Print(conn.Close())
		log.Print("after before conn.Close() 1")
		log.Print(conn.Close())
	}()
	data := make([]byte, 1024)
	_, err := conn.Read(data)
	if err != nil {
		log.Print(err)
	}
}

func main() {
	ln, err := net.Listen("tcp", ":1123")
	if err != nil {
		log.Fatal(err)
	}
	defer func() {
		log.Printf("ln.Close2: %s", ln.Close())
		time.Sleep(1 * time.Second)
	}()
	go func() {
		time.Sleep(10 * time.Second)
		log.Printf("ln.Close1: %s", ln.Close())
	}()
	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Printf("ln.Accept: %s", err)
			break
		}
		go serve(conn)
	}
}

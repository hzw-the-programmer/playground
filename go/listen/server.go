package main

import (
	"log"
	"net"
)

func main() {
	hub := newHub()
	go hub.run()

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
		client := newClient(hub, conn)
		go client.serve()
	}
}

type Hub struct {
	clients    map[*Client]bool
	register   chan *Client
	unregister chan *Client
	broadcast  chan []byte
}

func newHub() *Hub {
	return &Hub{
		clients:    make(map[*Client]bool),
		register:   make(chan *Client),
		unregister: make(chan *Client),
		broadcast:  make(chan []byte),
	}
}

func (hub *Hub) run() {
	for {
		select {
		case client := <-hub.register:
			hub.clients[client] = true
		case client := <-hub.unregister:
			delete(hub.clients, client)
		case data := <-hub.broadcast:
			for client := range hub.clients {
				client.writech <- data
			}
		}
	}
}

type Client struct {
	hub     *Hub
	conn    net.Conn
	writech chan []byte
}

func newClient(hub *Hub, conn net.Conn) *Client {
	return &Client{
		hub:     hub,
		conn:    conn,
		writech: make(chan []byte),
	}
}

func (client *Client) serve() {
	client.hub.register <- client
	go client.read()
	go client.write()
}

func (client *Client) read() {
	raddr := client.conn.RemoteAddr()
	data := make([]byte, 1024)
	for {
		n, err := client.conn.Read(data)
		if err != nil {
			log.Printf("%s: read error: %s", raddr, err)
			break
		}
		log.Printf("%s: read: %s", raddr, string(data[:n]))
		client.hub.broadcast <- data[:n]
	}
	log.Printf("%s: exist read", raddr)
	client.hub.unregister <- client
	client.conn.Close()
	close(client.writech)
}

func (client *Client) write() {
	raddr := client.conn.RemoteAddr()
	for data := range client.writech {
		n, err := client.conn.Write(data)
		if err != nil {
			log.Printf("%s: write error: %s", raddr, err)
			break
		}
		log.Printf("%s: write: %s", raddr, string(data[:n]))
	}
	log.Printf("%s: exist write", raddr)
}

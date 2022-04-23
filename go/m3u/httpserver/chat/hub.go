package chat

import (
	"log"
)

type Hub struct {
	register chan *Client
	unregister chan *Client
	clients map[*Client]bool
	broadcast chan []byte
}

func NewHub() *Hub {
	return &Hub{
		register: make(chan *Client),
		unregister: make(chan *Client),
		clients: make(map[*Client]bool),
		broadcast: make(chan []byte),
	}
}

func (hub *Hub) Run() {
	for {
		select {
		case client := <-hub.register:
			log.Print("hub: register client")
			hub.clients[client] = true
		case client := <-hub.unregister:
			log.Print("hub: unregister client")
			if _, ok := hub.clients[client]; ok {
				delete(hub.clients, client)
				close(client.send)
			}
		case msg := <-hub.broadcast:
			log.Printf("hub: broadcast msg: %s", msg)
			for client := range hub.clients {
				select {
				case client.send <- msg:
				default:
					close(client.send)
					delete(hub.clients, client)
				}
			}
		}
	}
}
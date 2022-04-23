package chat

import (
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

type Client struct {
	conn *websocket.Conn
	send chan []byte
	hub *Hub
}

func (client *Client) readPump() {
	defer func() {
		client.hub.unregister <- client
		log.Print("client: readPump exit")
	}()
	
	for {
		_, msg, err := client.conn.ReadMessage()
		if err != nil {
			log.Printf("client: readPump err: %v", err)
			return
		}
		log.Printf("client: receive msg: %s", msg)
		client.hub.broadcast <- msg
	}
}

func (client *Client) writePump() {
	defer func() {
		log.Print("client: writePump exit")
	}()
	
	for {
		select {
		case msg, ok := <-client.send:
			if !ok {
				log.Print("client: send chan closed")
				return
			}
			log.Printf("client: send msg: %s", msg)
			if err := client.conn.WriteMessage(websocket.TextMessage, msg); err != nil {
				log.Print(err)
			}
		}
	}
}

var upgrader = websocket.Upgrader{
	ReadBufferSize: 1024,
	WriteBufferSize: 1024,
}

func ServeWs(w http.ResponseWriter, r *http.Request, hub *Hub) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("client: upgrade err: %v", err)
		return
	}

	log.Print("client: connected")
	
	client := &Client{conn, make(chan []byte), hub}
	client.hub.register <- client
	go client.readPump()
	go client.writePump()
}
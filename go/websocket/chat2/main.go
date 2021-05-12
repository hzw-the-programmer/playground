package main

import (
	"log"
	"net/http"

	"golang.org/x/net/websocket"
)

func main() {
	hub := NewHub()
	go hub.Run()

	http.Handle("/chat", websocket.Handler(func(conn *websocket.Conn) {
		c := NewClient(conn)
		hub.Register(c)
		go c.ReadPump()
		go c.WritePump()
		for msg := range c.Readch() {
			// c.Writech() <- msg
			hub.Broadcast() <- msg
		}
		log.Print("exit handler")
	}))
	http.Handle("/", http.FileServer(http.Dir("web")))
	err := http.ListenAndServe(":1123", nil)
	if err != nil {
		log.Fatal(err)
	}
}

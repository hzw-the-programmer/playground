package main

import (
	"bufio"
	"flag"
	"log"
	"net/http"
	"os"

	"golang.org/x/net/websocket"
)

var user *string
var port *string

func main() {
	isClient := flag.Bool("c", false, "is client?")
	user = flag.String("u", "hzw", "user name")
	port = flag.String("p", "4444", "port")
	flag.Parse()

	if *isClient {
		client()
	} else {
		server()
	}
}

func client() {
	conn, err := websocket.Dial("ws://localhost:"+*port+"/chat1", "", "localhost:"+*port)
	if err != nil {
		log.Fatal(err)
	}
	go func() {
		for {
			var msg string
			if err := websocket.Message.Receive(conn, &msg); err == nil {
				log.Print(msg)
			}
		}
	}()
	s := bufio.NewScanner(os.Stdin)
	for s.Scan() {
		websocket.Message.Send(conn, s.Text())
	}
}

func chat1(conn *websocket.Conn) {
	for {
		var msg string
		if err := websocket.Message.Receive(conn, &msg); err == nil {
			log.Print(msg)
			websocket.Message.Send(conn, msg)
		}
	}
}

func server() {
	http.Handle("/chat1", websocket.Handler(chat1))
	if err := http.ListenAndServe(":"+*port, nil); err != nil {
		log.Fatal(err)
	}
}

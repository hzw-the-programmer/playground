package cmd

import (
	"log"
	"net/http"

	"github.com/spf13/cobra"
	"golang.org/x/net/websocket"
)

func echoHandler(ws *websocket.Conn) {
	for {
		msg := make([]byte, 512)
		n, err := ws.Read(msg)
		if err != nil {
			log.Printf("Read error: %s\n", err)
			break;
		}
		log.Printf("Received: %s\n", msg[:n])

		m, err := ws.Write(msg[:n])
		if err != nil {
			log.Printf("Write error: %s\n", err)
			break;
		}
		log.Printf("Sent: %s\n", msg[:m])
	}
}

func run(cmd *cobra.Command, args[]string) {
	http.Handle("/", http.FileServer(http.Dir("webroot")))
	http.Handle("/echo", websocket.Handler(echoHandler))
	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		panic("ListenAndServe: " + err.Error())
	}
}

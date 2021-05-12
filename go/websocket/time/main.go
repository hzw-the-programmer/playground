package main

import (
	"time"
	"net/http"

	"golang.org/x/net/websocket"
)

type Msg struct {
	Time int64
}

func timeh(conn *websocket.Conn) {
	ticker := time.NewTicker(time.Second)
	defer ticker.Stop()
	for {
		select {
		case t := <-ticker.C:
			msg := Msg{Time: t.Unix()}
			websocket.JSON.Send(conn, msg)
		}
	}
}

func main() {
	http.Handle("/time", websocket.Handler(timeh))
	http.ListenAndServe(":8000", nil)
}

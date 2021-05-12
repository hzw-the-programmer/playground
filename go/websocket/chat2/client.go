package main

import (
	"log"

	"golang.org/x/net/websocket"
)

type client struct {
	conn    *websocket.Conn
	readch  chan string
	writech chan string
	hub     *hub
}

func NewClient(conn *websocket.Conn) *client {
	return &client{
		conn:    conn,
		readch:  make(chan string),
		writech: make(chan string),
	}
}

func (c *client) SetHub(h *hub) {
	c.hub = h
}

func (c *client) ReadPump() {
	defer c.hub.Unregister(c)
	defer close(c.writech)
	defer close(c.readch)
	var msg string
	var err error
	for {
		err = websocket.Message.Receive(c.conn, &msg)
		if err != nil {
			log.Println("ReadPump", err)
			break
		}
		c.readch <- msg
	}
	log.Print("exit ReadPump")
}

func (c *client) WritePump() {
	var err error
	for msg := range c.writech {
		err = websocket.Message.Send(c.conn, msg)
		if err != nil {
			log.Println("WritePump", err)
			break
		}
	}
	log.Print("exit WritePump")
}

func (c *client) Readch() chan string {
	return c.readch
}

func (c *client) Writech() chan string {
	return c.writech
}

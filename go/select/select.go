package main

import (
	"log"
	"time"
)

type chcreator struct {
	name string
	ch   chan struct{}
}

func (c *chcreator) getch() <-chan struct{} {
	log.Printf("%s: getch called", c.name)
	// if c.ch == nil {
	// 	log.Printf("%s: create channel", c.name)
	// 	c.ch = make(chan struct{})
	// }
	return c.ch
}

func main() {
	c1 := chcreator{name: "c1"}
	c2 := chcreator{name: "c2"}
	for {
		select {
		default:
			log.Print("default executed")
		case <-c2.getch():
		case <-c1.getch():
		}
		log.Print("sleep")
		time.Sleep(1 * time.Second)
	}
}

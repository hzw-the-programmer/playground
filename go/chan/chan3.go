package main

import (
	"log"
)

func main() {
	c := make(chan []byte, 1)
	go func() {
		d := []byte{0, 1, 2}
		c <- d
		d[0] = 3
	}()
	log.Print(<-c)
}

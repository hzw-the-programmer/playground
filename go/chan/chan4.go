package main

import (
	"log"
	"time"
)

func main() {
	len := 10
	c := make(chan int, len)
	for i := 0; i < len; i++ {
		c <- i
	}

	close(c)

	for {
		v, ok := <-c
		log.Println(v, ok)
		time.Sleep(1 * time.Second)
	}
}

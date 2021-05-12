package main

import (
	"log"
	"time"
)

func main() {
	readch := make(chan int)
	writech := make(chan int)
	
	go func() {
		i := 0
		for {
			select {
			case <-time.Tick(1 * time.Second):
				readch <- i
				i++
			}
		}
	}()
	
	go func() {
		for i := range writech {
			log.Print(i)
		}
	}()

	for i := range readch {
		writech <- i
	}
}

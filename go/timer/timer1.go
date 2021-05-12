package main

import (
	"log"
	"time"
)

func main() {
	log.SetFlags(log.Ldate | log.Lmicroseconds)
	ticker := time.NewTicker(1 * time.Second)
	i := 0
	for {
		select {
		case t := <-ticker.C:
			log.Print(t)
			i++
			if i == 3 {
				time.Sleep(2*time.Second + 500*time.Millisecond)
			} else if i == 6 {
				ticker.Stop()
				return
			}
		}
	}
}

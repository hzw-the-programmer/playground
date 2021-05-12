package main

import (
	"log"
	"time"
)

func main() {
	now := time.Now()
	// defer log.Print(time.Since(now).Seconds())
	defer func() {
		log.Print(time.Since(now).Seconds())
	}()
	time.Sleep(1 * time.Second)
}

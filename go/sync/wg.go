package main

import (
	"sync"
	"time"
	"log"
)

func main() {
	var wg sync.WaitGroup
	for {
		wg.Add(1)
		go func() {
			defer wg.Done()
			time.Sleep(1 * time.Second)
		}()

		wg.Add(1)
		go func() {
			defer wg.Done()
			time.Sleep(2 * time.Second)
		}()
		
		wg.Wait()
		log.Print("after wait")
	}
}

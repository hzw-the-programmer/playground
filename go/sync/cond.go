package main

import (
	"log"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup
	var m sync.Mutex
	cond := sync.NewCond(&m)
	for i := 0; i < 3; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			cond.L.Lock()
			cond.Wait()
			log.Printf("%d wait returned", i)
			time.Sleep(10 * time.Second)
			cond.L.Unlock()
		}(i)
	}
	time.Sleep(1 * time.Second)
	// cond.L.Lock()

	log.Print("main broadcast")
	cond.Broadcast()
	// log.Print("main signal")
	// cond.Signal()

	// time.Sleep(1 * time.Second)
	log.Print("main unlock")
	// cond.L.Unlock()

	wg.Wait()
}

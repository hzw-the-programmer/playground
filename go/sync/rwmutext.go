package main

import (
	"log"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup
	var rwmu sync.RWMutex

	wg.Add(1)
	go func() {
		defer wg.Done()
		rwmu.RLock()
		defer rwmu.RUnlock()
		log.Print("r lock1 ok")
		time.Sleep(1 * time.Second)
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		rwmu.RLock()
		defer rwmu.RUnlock()
		log.Print("r lock2 ok")
		time.Sleep(1 * time.Second)
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		rwmu.Lock()
		defer rwmu.Unlock()
		log.Print("w lock1 ok")
		time.Sleep(2 * time.Second)
	}()

	wg.Add(1)
	go func() {
		defer wg.Done()
		rwmu.Lock()
		defer rwmu.Unlock()
		log.Print("w lock2 ok")
		time.Sleep(2 * time.Second)
	}()

	wg.Wait()
}

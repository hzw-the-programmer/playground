package main

import (
	"fmt"
	"sync"
	"time"
)

var rwl sync.RWMutex

func wl(wg *sync.WaitGroup) {
	defer wg.Done()
	fmt.Println("try Lock")
	rwl.Lock()
	defer rwl.Unlock()
	fmt.Println("got Lock")
	time.Sleep(2 * time.Second)
	fmt.Println("release Lock")
}

func rl(wg *sync.WaitGroup) {
	defer wg.Done()
	fmt.Println("try RLock")
	rwl.RLock()
	defer rwl.RUnlock()
	fmt.Println("got RLock")
	time.Sleep(2 * time.Second)
	fmt.Println("release RLock")
}

func main() {
	var wg sync.WaitGroup
	wg.Add(2)
	//go wl(&wg)
	go rl(&wg)
	time.Sleep(1 * time.Second)
	//rl(&wg)
	wl(&wg)
	wg.Wait()
}

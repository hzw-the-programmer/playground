package main

import (
	"fmt"
	"time"
	"sync"
)

var counter int
var mux sync.Mutex

func inc() {
	mux.Lock()
	temp := counter
	time.Sleep(time.Millisecond)
	temp++
	counter = temp
	mux.Unlock()
}

func main() {
	for i := 0; i < 100; i++ {
		go inc()
	}
	time.Sleep(time.Second)
	fmt.Println(counter)
}
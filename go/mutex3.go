package main

import (
	"fmt"
	"sync"
)

var x = 0

func inc(wg *sync.WaitGroup) {
	x = x + 1
	wg.Done()
}

func main() {
	var wg sync.WaitGroup
	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go inc(&wg)
	}
	wg.Wait()
	fmt.Println(x)
}
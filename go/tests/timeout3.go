package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	time.AfterFunc(1*time.Second, func() {
		defer wg.Done()
		fmt.Println("timeout")
	})
	wg.Wait()
}

package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	//reqChan := make(chan int)
	reqChan := make(chan int, 5)

	go makeReqs(reqChan)
	serveReqs(reqChan)
}

func makeReqs(reqChan chan<- int) {
	for i := 0; i < 10; i++ {
		reqChan <- i
		fmt.Println("make req", i)
	}
	close(reqChan)
}

func serveReqs(reqChan <-chan int) {
	var wg sync.WaitGroup
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			handleReqs(reqChan)
		}()
	}
	wg.Wait()
}

func handleReqs(reqChan <-chan int) {
	for req := range reqChan {
		handleReq(req)
	}
}

func handleReq(req int) {
	fmt.Println("handle", req)
	time.Sleep(10 * time.Second)
}

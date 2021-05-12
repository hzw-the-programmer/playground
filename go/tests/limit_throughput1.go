package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	reqChan := make(chan int)

	go makeReqs(reqChan)
	serveReqs(reqChan)
}

func makeReqs(reqChan chan<- int) {
	for i := 0; i < 10; i++ {
		fmt.Println("make req", i)
		reqChan <- i
	}
	close(reqChan)
}

func serveReqs(reqChan <-chan int) {
	sem := make(chan int, 5)
	var wg sync.WaitGroup
	for req := range reqChan {
		wg.Add(1)
		go func(req int) {
			defer wg.Done()
			fmt.Println("handle enter", req)
			sem <- 1
			fmt.Println("handle begin", req)
			handleReq(req)
			<-sem
			fmt.Println("handle end", req)
		}(req)
	}
	wg.Wait()
}

func handleReq(req int) {
	time.Sleep(10 * time.Second)
}

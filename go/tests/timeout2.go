package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup

	rc := make(chan bool)
	//rc := make(chan bool, 1)
	wg.Add(1)
	go func() {
		defer wg.Done()
		time.Sleep(2 * time.Second)
		/*
			select {
			case rc <- true:
			default:
			}
		*/
		rc <- true
		fmt.Println("rc finish")
	}()

	select {
	case <-rc:
		fmt.Println("rc return")
	case <-time.After(1 * time.Second):
		fmt.Println("timeout")
	}

	wg.Wait()
	//time.Sleep(4 * time.Second)
	//for {}
}

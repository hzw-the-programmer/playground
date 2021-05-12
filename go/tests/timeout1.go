package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	var wg sync.WaitGroup

	//rc := make(chan bool)
	rc := make(chan bool, 1)
	wg.Add(1)
	go func() {
		defer wg.Done()
		//time.Sleep(1 * time.Second)
		time.Sleep(2 * time.Second)
		//select {
		//case rc <- true:
		//default:
		//}
		rc <- true
		fmt.Println("rc finish")
	}()

	tc := make(chan bool)
	//tc := make(chan bool, 1)
	wg.Add(1)
	go func() {
		defer wg.Done()
		//time.Sleep(2 * time.Second)
		time.Sleep(1 * time.Second)
		select {
		case tc <- true:
		default:
		}
		fmt.Println("tc finish")
	}()

	select {
	case <-rc:
		fmt.Println("rc return")
	case <-tc:
		fmt.Println("tc return")
	}

	wg.Wait()
}

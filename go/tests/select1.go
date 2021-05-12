package main

import (
	"fmt"
	"time"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	ch1 := make(chan int)
	ch2 := make(chan int)
	wg.Add(1)
	go sel(ch1, ch2, wg)
	time.Sleep(1 * time.Second)
	close(ch1)
	/*
	i := 0
	for {
		ch1 <- i
		i++
	}
	*/
	wg.Wait()
}

func sel(ch1, ch2 chan int, wg sync.WaitGroup) {
	defer wg.Done()
	j := 1
	for {
		select {
		case i, ok := <-ch1:
			fmt.Println("ch1", i, ok, j)
			//time.Sleep(1 * time.Second)
		case i, ok := <-ch2:
			fmt.Println("ch2", i, ok)
		}
		j++
	}
}

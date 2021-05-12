package main

import (
	"fmt"
	"time"
	"sync"
)

func main() {
	c := make(chan int)
	var wg sync.WaitGroup

	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(c chan int, i int) {
			fmt.Println("before write", i)
			c <- i
			fmt.Println("after write", i)
			wg.Done()
		}(c, i)
	}

	go func() {
		wg.Wait()
		close(c)
	}()

	time.Sleep(1 * time.Millisecond)

	/*
	for i := 0; i < 10; i++ {
		fmt.Println("before read", i)
		<-c
		fmt.Println("after read", i)
	}

	time.Sleep(1 * time.Millisecond)
	*/

	for i := range c {
		fmt.Println(i)
	}
}
package main

import (
	"fmt"
)

func main() {
	c := make(chan int)
	go func() {
		fmt.Println("before send.")
		c <- 1
		fmt.Println("after send.")
	}()
	select {
	case <-c:
		fmt.Println("received.")
	}
}
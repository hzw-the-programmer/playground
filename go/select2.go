package main

import (
	"fmt"
	//"time"
)

func main() {
	c0 := make(chan int)
	//c0 := make(chan int, 1)
	c1 := make(chan int)

	go func() {
		fmt.Println("before send to c1.")
		c1<-1
		fmt.Println("after send to c1.")

		fmt.Println("before send to c0.")
		c0<-0
		fmt.Println("after send to c0.")

		/*
		fmt.Println("before send to c1.")
		c1<-1
		fmt.Println("after send to c1.")
		*/
	}()

	select {
	case <-c0:
		fmt.Println("received from c0.")
	case <-c1:
		fmt.Println("received from c1.")
	}

	/*
	select {
	case <-c0:
		fmt.Println("received from c0.")
	case <-c1:
		fmt.Println("received from c1.")
	}

	time.Sleep(time.Millisecond)
	*/
}
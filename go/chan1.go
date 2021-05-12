package main

import (
	"fmt"
	"time"
)

func main() {
	c := make(chan int)
	
	go func(c chan int) {
		/*
		fmt.Println("before write 1")
		c <- 1
		fmt.Println("after write 1")
		fmt.Println("before write 2")
		c <- 2
		fmt.Println("after write 2")
		*/
		fmt.Println("before read 1")
		<-c
		fmt.Println("after read 1")
		fmt.Println("before read 2")
		<-c
		fmt.Println("after read 2")
	}(c)

	/*
	fmt.Println("before read 1")
	<-c
	fmt.Println("after read 1")
	fmt.Println("before read 2")
	<-c
	fmt.Println("after read 2")
	*/
	fmt.Println("before write 1")
	c <- 1
	fmt.Println("after write 1")
	fmt.Println("before write 2")
	c <- 2
	fmt.Println("after write 2")

	time.Sleep(1 * time.Millisecond)
}
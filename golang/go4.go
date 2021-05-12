package main

import (
	"fmt"
	"time"
)

//func pinger(c chan string) {
func pinger(c chan<- string) {
	for i := 0; ; i++ {
		s := fmt.Sprintf("ping %d", i)
		fmt.Println("send", s)
		c <- s
	}
}

//func printer(c chan string) {
func printer(c <-chan string) {
	for {
		msg := <- c
		fmt.Println("receive", msg)
		time.Sleep(time.Second * 10)
	}
}

func ponger(c chan<- string) {
	for i := 0; ; i++ {
		s := fmt.Sprintf("pong %d", i)
		fmt.Println("send", s)
		c <- s
	}
}

func main() {
	c := make(chan string)

	//go printer(c)
	go pinger(c)
	go ponger(c)
	go printer(c)

	fmt.Println("wait for input")
	var input string
	fmt.Scanln(&input)
}
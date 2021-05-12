package main

import (
	"fmt"
	"time"
)

func main() {
	c1 := make(chan string)
	c2 := make(chan string)

	go func() {
		for i := 0; ; i++ {
			fmt.Println("before from 2")

			s := fmt.Sprintf("from 2 %d", i)
			fmt.Println("******************")
			c2 <- s
			fmt.Println("++++++++++++++++++")
			fmt.Println(fmt.Sprintf("%s sent", s))
			
			time.Sleep(time.Second * 3)
			
			fmt.Println("after from 2")
		}
	}()

	go func() {
		for {
			fmt.Println("before select")
			select {
			case msg2 := <- c2:
				fmt.Println(msg2)
			case msg1 := <- c1:
				fmt.Println(msg1)
			//case msg2 := <- c2:
			//	fmt.Println(msg2)
			}
			fmt.Println("after select")
		}
	}()
	
	go func() {
		for i := 0; ; i++ {
			fmt.Println("before from 1")

			s := fmt.Sprintf("from 1 %d", i)
			c1 <- s
			fmt.Println(fmt.Sprintf("%s sent", s))
			
			time.Sleep(time.Second * 2)

			fmt.Println("after from 1")
		}
	}()

	/*
	go func() {
		for i := 0; ; i++ {
			fmt.Println("before from 2")

			s := fmt.Sprintf("from 2 %d", i)
			fmt.Println("******************")
			c2 <- s
			fmt.Println("++++++++++++++++++")
			fmt.Println(fmt.Sprintf("%s sent", s))
			
			time.Sleep(time.Second * 3)
			
			fmt.Println("after from 2")
		}
	}()
	*/

	/*
	go func() {
		for {
			fmt.Println("before select")
			select {
			case msg2 := <- c2:
				fmt.Println(msg2)
			case msg1 := <- c1:
				fmt.Println(msg1)
			//case msg2 := <- c2:
			//	fmt.Println(msg2)
			}
			fmt.Println("after select")
		}
	}()
	*/

	var input string
	fmt.Scanln(&input)
}
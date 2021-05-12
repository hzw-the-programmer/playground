package main

import (
	"fmt"
	"time"
)

func main() {
	/*
	for i := 0; i < 10; i++ {
		go func() {
			fmt.Println(i)
		}()
	}
	*/

	/*
	for i := 0; i < 10; i++ {
		go func(i int) {
			fmt.Println(i)
		}(i)
	}
	*/

	/*
	//j := 0
	var j int
	for i := 0; i < 10; i++ {
		j = i
		go func() {
			fmt.Println(j)
		}()
	}
	*/

	for i := 0; i < 10; i++ {
		//j := i
		var j int = i
		go func() {
			fmt.Println(j)
		}()
	}

	time.Sleep(5 * time.Millisecond)
}
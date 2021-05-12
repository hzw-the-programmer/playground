package main

import (
	"fmt"
)

func gen() <-chan int {
	return nil
}

func main() {
	//var c chan int
	select {
	case <-gen():
	//case <-c:
		fmt.Println("received from nil chan")
	default:
		fmt.Println("receive from nil chan blocked")
	}
}

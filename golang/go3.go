package main

import (
	"fmt"
	"time"
	"math/rand"
)

func f(n int) {
	for i := 0; i < 3; i++ {
		fmt.Println(n, ":", i)
		amt := time.Duration(rand.Intn(250))
		time.Sleep(time.Millisecond * amt)
	}
}

func main() {
	for i := 0; i < 5; i++ {
		go f(i)
	}
	fmt.Println("wait for input")
	var input string
	fmt.Scanln(&input)
}
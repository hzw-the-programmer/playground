package main

import "fmt"

func f(n int) {
	for i := 0; i < 3; i++ {
		fmt.Println(n, ":", i)
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
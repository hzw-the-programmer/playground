package main

import "fmt"

func main() {
	var x [5]int
	fmt.Println(x)
	x[4] = 100
	fmt.Println(x)
}
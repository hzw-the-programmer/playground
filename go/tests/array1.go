package main

import (
	"fmt"
)

func main() {
	//a := [2]int{1, 2}
	a := []int{1, 2}
	fmt.Printf("%T\n", a)
	b := a
	b[0] = 2
	fmt.Println(a, b)
}

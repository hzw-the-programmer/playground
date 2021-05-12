package main

import (
	"fmt"
)

func main() {
	/*
	a := []int{1, 2}
	b := a
	a = append(a, 3)
	b[0] = 99
	fmt.Println(a, b)
	*/
	a := make([]int, 2, 3)
	a[0] = 1
	a[1] = 2
	b := a
	a = append(a, 3)
	b[0] = 99
	fmt.Println(a, b)
}

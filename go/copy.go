package main

import (
	"fmt"
)

func main() {
	a := []int{1, 2, 3, 4, 5}

	b := a[1: 3]

	c := []int{6, 7, 8, 9, 10}

	fmt.Println(len(b), cap(b))
	n := copy(b, c)
	fmt.Println(n, a)
	fmt.Println(len(b), cap(b))
}

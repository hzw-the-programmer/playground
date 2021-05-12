package main

import (
	"fmt"
)

func main() {
	a := []int{1, 2, 3, 4, 5}

	b := a[1:3]
	fmt.Println(len(b), cap(b))
	fmt.Println(b)
	fmt.Println(a)

	b = append(b, 6)
	//b = append(b, 6, 7, 8)
	fmt.Println(len(b), cap(b))
	fmt.Println(b)
	fmt.Println(a)

	b = append(b, 7)
	fmt.Println(len(b), cap(b))
	fmt.Println(b)
	fmt.Println(a)

	b = append(b, 8)
	fmt.Println(len(b), cap(b))
	fmt.Println(b)
	fmt.Println(a)
}

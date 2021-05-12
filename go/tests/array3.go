package main

import (
	"fmt"
)

func main() {
	/*
	a := [...]int{1, 2, 3}
	//b := [...]int{1, 2, 3}
	b := a
	b[0] = 2
	a[0] = 2
	fmt.Printf("%T %T\n", a, b)
	if a == b {
		fmt.Println("a == b true")
	} else {
		fmt.Println("a == b false")
	}
	*/

	/*
	a := []int{1, 2, 3}
	b := []int{1, 2, 3}
	fmt.Printf("%T %T\n", a, b)
	if a == b {
		fmt.Println("a == b true")
	} else {
		fmt.Println("a == b false")
	}
	*/

	a := [...]int{1, 2, 3}
	b := a[:]
	b[0] = 2
	fmt.Printf("%T %T\n", a, b)
	fmt.Println(a, b)
	/*
	if a == b {
		fmt.Println("a == b true")
	} else {
		fmt.Println("a == b false")
	}
	*/
}

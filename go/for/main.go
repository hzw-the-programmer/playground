package main

import (
	"fmt"
)

func main() {
	r := [][1]int{}
	a := [][1]int{[1]int{1}, [1]int{2}, [1]int{3}}
	for _, i := range a {
		fmt.Printf("%T\n", i)
		i[0] += 1
		fmt.Println(i[:])
		r = append(r, i)
		i[0] += 1
	}
	fmt.Println(a)
	fmt.Println(r)

	aa := [1]int{1}
	bb := aa[:]
	fmt.Println(bb)
	aa = [1]int{2}
	fmt.Println(bb)
}

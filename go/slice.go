package main

import (
	"fmt"
)

func main() {
	s1 := make([]byte, 2, 4)
	fmt.Printf("len: %d, cap: %d\n", len(s1), cap(s1))
	
	s1[0] = 0
	s1[1] = 1
	//s1[2] = 2
	
	s1 = s1[:4]
	fmt.Printf("len: %d, cap: %d\n", len(s1), cap(s1))

	s1[2] = 2
	s1[3] = 3
	//s1[4] = 4

	//s1 = s1[:6]
	//fmt.Printf("len: %d, cap: %d\n", len(s1), cap(s1))
	fmt.Println(s1)

	s2 := make([]byte, 2, 4)
	fmt.Printf("%d\n", copy(s2, s1))
	fmt.Println(s2)
	fmt.Println(s2[:4])
	fmt.Printf("%d\n", copy(s2[:4], s1))
	fmt.Println(s2)
	fmt.Println(s2[:4])
}
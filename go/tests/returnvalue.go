package main

import (
	"fmt"
)

type s1 struct {
	a, b int
}

func ns1() *s1 {
	return &s1{1, 2}
}
/*
func ns1() s1 {
	return s1{1, 2}
}
*/

func main() {
	s := ns1()
	f := s
	f.a = 2
	fmt.Println(s, f)
}

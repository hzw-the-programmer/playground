package main

import (
	"fmt"
)

func main() {
	s1()
	s2()
	s3()
	f1, f2 := s4()
	f1()
	f2()
	f1()
	f2()
}

func s1() {
	i := 99
	for i := 0; i < 10; i++ {
		fmt.Println(i)
	}
	fmt.Println(i)
}

func s2() {
	i := 100
	if i := 23; i != 23 {
		fmt.Println("i != 23")
	} else {
		fmt.Println(i)
	}
	fmt.Println(i)
}

func s3() {
	i := 101

	switch i := 10; i {
	case 10:
		i := 11
		fmt.Println("in case 10", i)
	default:
		fmt.Println("in default", i)
	}

	fmt.Println("after switch", i)
}

func s4() (func(), func()) {
	i := 111
	return func() {
			i += 1
		}, func() {
			fmt.Println(i)
		}
}

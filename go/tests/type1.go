package main

import (
	"fmt"
)

type DevAddr [4]byte

func f1(a [4]byte) {
	fmt.Println("f1")
	a[0] = 1
}

func f2(s []byte) {
	fmt.Println("f2")
	s[0] = 1
}

func main() {
	var a DevAddr
	fmt.Println(a)
	f1(a)
	fmt.Println(a)
	f2(a[:])
	fmt.Println(a)
}

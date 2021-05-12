package main

import (
	"log"
)

func main() {
	a := []byte{1, 2, 3}
	b := a[2:]
	c := a[3:]
	//d := a[4:]
	//log.Print(a, b, c, d)
	log.Print(a, b, c)
}
package main

import (
	"log"
)

func main() {
	var b byte = 0x31
	c := '1'
	log.Printf("%T, '%c'", b, b)
	log.Printf("%T, 0x%x", c, c)
	log.Print(rune(b) == c)
}

package main

import (
	"log"
)

func main() {
	b := byte(1)
	c := b << 8
	d := uint16(b << 8)
	e := uint16(b) << 8
	f := byte(1)
	g := f<<9 | b<<8
	h := uint16(f)<<9 | uint16(b)<<8
	log.Print(c, d, e, g, h)
}

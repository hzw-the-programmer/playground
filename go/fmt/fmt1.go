package main

import (
	"fmt"
)

func main() {
	c1 := '_'
	c2 := 0x5f
	c3 := 95
	s := "abc"
	fmt.Printf("%x %X %q %T\n", c1, c1, c1, c1)
	fmt.Printf("%x %X %q %T\n", c2, c2, c2, c2)
	fmt.Printf("%x %X %q %T\n", c3, c3, c3, c3)
	fmt.Printf("%s %q\n", s, s)
}

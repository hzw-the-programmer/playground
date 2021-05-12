package main

import (
	"fmt"
	"unicode/utf8"
	"os"
	"strconv"
)

func main() {
	b := [512]byte{}
	r, err := strconv.ParseInt(os.Args[1], 16, 64)
	if err != nil {
		panic(err)
	}
	l := utf8.EncodeRune(b[:], rune(r))
	fmt.Printf("%x\n", b[:l])
}

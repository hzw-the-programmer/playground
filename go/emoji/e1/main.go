package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	str := "😄 🐷"
	fmt.Printf("len=%d\n", len(str))
	fmt.Printf("len=%d\n", utf8.RuneCountInString(str))
	for i, c := range str {
		fmt.Printf("%d: %08x, %T\n", i, c, c)
	}

	runes := []rune(str)
	fmt.Printf("len=%d\n", len(runes))
	for i, r := range runes {
		fmt.Printf("%d: %08x, %T\n", i, r, r)
	}
}

package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	const sample = "\xbd\xb2\x3d\xbc\x20\xe2\x8c\x98"

	fmt.Println(sample)

	for i := 0; i < len(sample); i++ {
		fmt.Printf("%x ", sample[i])
	}
	fmt.Println()

	fmt.Printf("%x\n", sample)

	fmt.Printf("% x\n", sample)

	fmt.Printf("%q\n", sample)

	fmt.Printf("%+q\n", sample)

	for i, r := range sample {
		fmt.Printf("%#U starts at byte position %d\n", r, i)
	}
	fmt.Println()

	const nihongo = "日本語"

	for i, r := range nihongo {
		fmt.Printf("%#U starts at byte position %d\n", r, i)
	}
	fmt.Println()
	for i, w := 0, 0; i < len(nihongo); i += w {
		r, width := utf8.DecodeRuneInString(nihongo[i:])
		fmt.Printf("%#U starts at byte position %d\n", r, i)
		w = width
	}
}

package main

import (
	"log"
	"unicode/utf8"
)

func main() {
	bee := '\U0001F41D'
	buf := make([]byte, 10)
	n := utf8.EncodeRune(buf, bee)
	log.Printf("%02x", buf[:n])
}

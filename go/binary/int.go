package main

import (
	"encoding/binary"
	"encoding/hex"
	"log"
)

func main() {
	a := -23
	b := make([]byte, 2)
	binary.BigEndian.PutUint16(b, uint16(a))
	log.Print(hex.EncodeToString(b))
	c := binary.BigEndian.Uint16(b)
	log.Print(int16(c))
	d := 0xffe9
	log.Print(int16(d))
}

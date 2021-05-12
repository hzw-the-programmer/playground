package main

import (
	"bytes"
	"encoding/binary"
	"encoding/hex"
	"log"
)

func main() {
	bs := []byte{0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x12, 0x34}
	bb := bytes.NewBuffer(bs)

	var a, b uint32
	var c, d uint16

	binary.Read(bb, binary.BigEndian, &a)
	binary.Read(bb, binary.LittleEndian, &b)
	binary.Read(bb, binary.BigEndian, &c)
	binary.Read(bb, binary.LittleEndian, &d)

	log.Printf("0x%08x, 0x%08x, 0x%04x, 0x%04x", a, b, c, d)

	binary.Write(bb, binary.BigEndian, &a)
	binary.Write(bb, binary.LittleEndian, &b)
	binary.Write(bb, binary.BigEndian, &c)
	binary.Write(bb, binary.LittleEndian, &d)

	log.Print(hex.EncodeToString(bb.Bytes()))
}

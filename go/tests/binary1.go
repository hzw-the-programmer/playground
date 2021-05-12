package main

import (
	"bytes"
	"encoding/binary"
	"encoding/hex"
	"log"
)

type A struct {
	// u16 uint16
	// u32 uint32
	U16 uint16
	U32 uint32
}

type B struct {
	//u8 uint8
	U8 uint8
	//a   *A
	//a   A
	A A
	//u64 uint64
	U64 uint64
}

func main() {
	b := B{
		0x01,
		//&A{
		A{
			0x0203,
			0x04050607,
		},
		0x0809101112131415,
	}
	//b := 0x1234
	//var b uint16 = 0x1234
	//var b int16 = 0x1234
	var bb bytes.Buffer
	err := binary.Write(&bb, binary.LittleEndian, &b)
	if err != nil {
		log.Print(err)
	}
	log.Print(hex.EncodeToString(bb.Bytes()))

	b1 := B{}
	binary.Read(&bb, binary.LittleEndian, &b1)
	log.Printf("%02x %04x %08x %016x", b1.U8, b1.A.U16, b1.A.U32, b1.U64)
}

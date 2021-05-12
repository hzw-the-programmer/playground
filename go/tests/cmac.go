package main

import (
	"fmt"
	"encoding/hex"

	"github.com/jacobsa/crypto/cmac"
)

func main() {
	b0 := make([]byte, 16)
	
	b0[0] = 0x49
	
	b0[6] = 0x04
	b0[7] = 0x03
	b0[8] = 0x02
	b0[9] = 0x01

	b0[10] = 0x01
	b0[11] = 0x00
	b0[12] = 0x00
	b0[13] = 0x00

	b0[15] = 14

	hash, _ := cmac.New([]byte{2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2})
	hash.Write(b0)
	hash.Write([]byte{
		0x40,
		0x04, 0x03, 0x02, 0x01,
		0x80,
		0x01, 0x00,
		0x01,
		0xa6, 0x94, 0x64, 0x26, 0x15,
	})
	/*
	hash.Write([]byte{
		64,
		4, 3, 2, 1,
		128,
		1, 0,
		1,
		166, 148, 100, 38, 21,
	})
	*/

	fmt.Println(hex.EncodeToString(hash.Sum([]byte{})))
}

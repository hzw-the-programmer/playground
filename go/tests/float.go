package main

import (
	"encoding/binary"
	"encoding/hex"
	"log"
	"math"
	"os"
)

func main() {
	bs, err := hex.DecodeString(os.Args[1])
	if err != nil {
		panic(err)
	}
	ui32 := binary.LittleEndian.Uint32(bs)
	f32 := math.Float32frombits(ui32)
	log.Print(f32)
}
